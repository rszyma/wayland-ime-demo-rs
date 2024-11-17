use std::{thread::sleep, time::Duration};

use wayland_client::{
    delegate_noop,
    protocol::{
        wl_registry,
        wl_seat::{self},
    },
    Connection, Dispatch, QueueHandle,
};

use wayland_protocols_misc::zwp_input_method_v2::client::{
    zwp_input_method_manager_v2, zwp_input_method_v2,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect_to_env().unwrap();
    let mut event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let display = conn.display();
    display.get_registry(&qhandle, ());

    let mut state = State {
        running: true,
        seat: None,
        input_method_manager: None,
        input_method: None,
    };

    let mut i = 0;
    while state.running {
        event_queue.roundtrip(&mut state).unwrap();
        conn.flush()?;
        println!("{i}");
        i += 1;
        sleep(Duration::from_millis(200));
        if i == 5 {
            println!("creating input method");
            state.create_input_method(&qhandle)?;
        } else if i % 20 == 19 {
            println!("sending text!");
            state.try_send_text("🥦")?;
        }
    }
    Ok(())
}

struct State {
    running: bool,
    seat: Option<wl_seat::WlSeat>,
    input_method_manager: Option<zwp_input_method_manager_v2::ZwpInputMethodManagerV2>,
    input_method: Option<zwp_input_method_v2::ZwpInputMethodV2>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            match &interface[..] {
                "wl_seat" => {
                    println!("handling event for wl_seat");
                    let seat = registry.bind::<wl_seat::WlSeat, _, _>(name, 1, qh, ());
                    state.seat = Some(seat);
                }
                "zwp_input_method_manager_v2" => {
                    println!("handling event for zwp_input_method_manager_v2");
                    let input_method_manager = registry
                        .bind::<zwp_input_method_manager_v2::ZwpInputMethodManagerV2, _, _>(
                        name,
                        1,
                        qh,
                        (),
                    );

                    state.input_method_manager = Some(input_method_manager);
                }
                _ => {}
            }
        }
    }
}

// Ignore events from these object types in this example.
delegate_noop!(State: ignore zwp_input_method_manager_v2::ZwpInputMethodManagerV2); // no events to handle
delegate_noop!(State: ignore wl_seat::WlSeat);

impl State {
    fn create_input_method(
        &mut self,
        qh: &QueueHandle<State>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let input_method_manager = self
            .input_method_manager
            .clone()
            .ok_or("input_method_manager not set")?;
        let seat = self.seat.clone().ok_or("seat not set")?;

        let input_method = input_method_manager.get_input_method(&seat, qh, ());
        println!("input_method: {:?}", input_method);

        self.input_method = Some(input_method);
        Ok(())
    }

    fn try_send_text(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let input_method = self.input_method.clone().ok_or("input_method not set")?;
        input_method.commit_string(text.to_string());
        // FIXME: pretty sure we should keep track of serial number.
        // Sending whatever number works on Hyprland though..
        input_method.commit(0);
        Ok(())
    }
}

impl Dispatch<zwp_input_method_v2::ZwpInputMethodV2, ()> for State {
    fn event(
        _: &mut Self,
        _: &zwp_input_method_v2::ZwpInputMethodV2,
        event: <zwp_input_method_v2::ZwpInputMethodV2 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("zwp_input_method_v2 event: {:?}", &event);
    }
}
