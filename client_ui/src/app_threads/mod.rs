use tokio::{runtime::{Handle, Runtime}, time::sleep};



pub fn tt_thread(app_handle: &Handle) {
    app_handle.spawn(async move {
        let mut count: i32 = 0;

        loop {
            count += 1;

            // let app2 = app.clone();   // ✅ invoke에 넘길 복제본
            let value = count;        // ✅ Copy라 그냥 값 전달

            // let _ = slint::invoke_from_event_loop(move || {
            //     if let Some(ui) = app2.upgrade() {
            //         let mut num =ui.get_test_counter1();
            //         num+=1;
            //         ui.set_test_counter1(num);
            //     }
            // });
            println!("{}",count);
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
    });
}