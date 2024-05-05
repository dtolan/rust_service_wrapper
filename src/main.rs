// Import the required dependencies.


//use std::fmt;
/*
// Top level struct to hold the TOML data.
#[derive(Deserialize)]
struct Data {
    config: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
struct Config {
    script: String,
}
*/
#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    ping_service::run()
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
/*
fn main() {
    // Variable that holds the filename as a `&str`.
    let filename = "c:\\config\\test.toml";

    // Read the contents of the file using a `match` block 
    // to return the `data: Ok(c)` as a `String` 
    // or handle any `errors: Err(_)`.
    let contents = match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    // Use a `match` block to return the 
    // file `contents` as a `Data struct: Ok(d)`
    // or handle any `errors: Err(_)`.
    let data: Data = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };
    let script = data.config.script;
    // Print out the values to `stdout`.
    println!("{}", script);
    //let create_shortcut = include_str!("script.ps1");
    let output = Command::new("powershell.exe")
            .args([script])
            .output()
            .expect("failed to execute process");
    let hello = output.stdout;
    let output = match String::from_utf8(hello) {
        Ok(string) =>  string,
        Err(_) => "Unable to convert to string".to_owned(),
    
    };
    println!("{}",output)


}
*/
#[cfg(windows)]
mod ping_service {
    /*

    use std::{
        ffi::OsString,
        //net::{IpAddr, SocketAddr, UdpSocket},
        sync::mpsc,
        time::Duration,
    };
    */
    use std::{
        fs,
        ffi::OsString,
        process::exit,
        sync::mpsc,
        time::Duration,
    };
    use serde_derive::Deserialize;
    use windows_service::{
        define_windows_service,
        service::{
            ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
            ServiceType,
        },
        service_control_handler::{self, ServiceControlHandlerResult},
        service_dispatcher, Result,
    };
    #[derive(Deserialize)]
    struct Data {
        config: Config,
    }

    // Config struct holds to data from the `[config]` section.
    #[derive(Deserialize)]
    struct Config {
        script: String,
    }
    const SERVICE_NAME: &str = "ping_service";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;
    /*
    const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
    const RECEIVER_PORT: u16 = 1234;
    const PING_MESSAGE: &str = "ping\n";
    */
    pub fn run() -> Result<()> {
        // Register generated `ffi_service_main` with the system and start the service, blocking
        // this thread until the service is stopped.
        service_dispatcher::start(SERVICE_NAME, ffi_service_main)
    }

    // Generate the windows service boilerplate.
    // The boilerplate contains the low-level service entry function (ffi_service_main) that parses
    // incoming service arguments into Vec<OsString> and passes them to user defined service
    // entry (my_service_main).
    define_windows_service!(ffi_service_main, my_service_main);

    // Service entry function which is called on background thread by the system with service
    // parameters. There is no stdout or stderr at this point so make sure to configure the log
    // output to file if needed.
    pub fn my_service_main(_arguments: Vec<OsString>) {
        if let Err(_e) = run_service() {
            // Handle the error, by logging or something.
        }
    }

    pub fn run_service() -> Result<()> {
        // Create a channel to be able to poll a stop event from the service worker loop.
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // Define system service event handler that will be receiving service events.
        let event_handler = move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                // Notifies a service to report its current status information to the service
                // control manager. Always return NoError even if not implemented.
                ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

                // Handle stop
                ServiceControl::Stop => {
                    shutdown_tx.send(()).unwrap();
                    ServiceControlHandlerResult::NoError
                }

                // treat the UserEvent as a stop request
                ServiceControl::UserEvent(code) => {
                    if code.to_raw() == 130 {
                        shutdown_tx.send(()).unwrap();
                    }
                    ServiceControlHandlerResult::NoError
                }

                _ => ServiceControlHandlerResult::NotImplemented,
            }
        };

        // Register system service event handler.
        // The returned status handle should be used to report service status changes to the system.
        let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

        // Tell the system that service is running
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        // Variable that holds the filename as a `&str`.
        let filename = "c:\\config\\test.toml";

        // Read the contents of the file using a `match` block 
        // to return the `data: Ok(c)` as a `String` 
        // or handle any `errors: Err(_)`.
        let contents = match fs::read_to_string(filename) {
            // If successful return the files text as `contents`.
            // `c` is a local variable.
            Ok(c) => c,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                eprintln!("Could not read file `{}`", filename);
                exit(1);
            }
        };

        // Use a `match` block to return the 
        // file `contents` as a `Data struct: Ok(d)`
        // or handle any `errors: Err(_)`.
        let data: Data = match toml::from_str(&contents) {
            // If successful, return data as `Data` struct.
            // `d` is a local variable.
            Ok(d) => d,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                eprintln!("Unable to load data from `{}`", filename);
                // Exit the program with exit code `1`.
                exit(1);
            }
        };
        let script = data.config.script;
        // Print out the values to `stdout`.
        println!("{}", script);


        loop {

            // Poll shutdown event.
            match shutdown_rx.recv_timeout(Duration::from_secs(1)) {
                // Break the loop either upon stop or channel disconnect
                Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

                // Continue work if no events were received within the timeout
                Err(mpsc::RecvTimeoutError::Timeout) => (),
            };
        }



        // Tell the system that service has stopped.
        status_handle.set_service_status(ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
            process_id: None,
        })?;

        Ok(())
    }
}