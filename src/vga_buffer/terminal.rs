// use crate::{print, println};
// use alloc::{string::String};
// use volatile::Volatile;
// use conquer_once::spin::OnceCell;
// use crossbeam_queue::ArrayQueue;
// use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
// use futures_util::stream::Stream;

// static COMMAND_CHAR_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

// pub struct TerminalCommand {
//     command_buffer: OnceCell<ArrayQueue<u8>>,
// }

// impl TerminalCommand {
//     pub fn new() -> Self {
//         COMMAND_CHAR_QUEUE.try_init_once(|| ArrayQueue::new(100))
//             .expect("TerminalCommand::new should only be called once");
//         TerminalCommand { command_buffer: OnceCell::uninit() }
//     }

//     pub fn add_scancode_to_command(&mut self, scancode: u8) {
//         if let Ok(queue) = self.command_buffer.try_get() {
//             if let Err(_) = queue.push(scancode) {
//                 println!("WARNING: command queue full; dropping keyboard input");
//             }
//         } else {
//             println!("WARNING: scancode queue uninitialized");
//         }
//     }

//     pub fn command_to_string(&mut self) -> String {
//         let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
//             HandleControl::Ignore);

//         let mut command_string = String::new();
        
//         self.command_buffer.try_get().into_iter().len()

//         while let Some(scancode) = scancodes.next().await {
//             if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
//                 if let Some(key) = keyboard.process_keyevent(key_event) {
//                     match key {
//                         DecodedKey::Unicode(character) => print!("{}", character),
//                         DecodedKey::RawKey(key) => print!("{:?}", key),
//                     }
//                 }
//             }
//         }

//         for scancode in self.command_buffer.try_get().into_iter() {
//             if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
//                 if let Some(key) = keyboard.process_keyevent(key_event) {
//                     match key {
//                         // DecodedKey::Unicode(character) => print!("{}", character),
//                         DecodedKey::Unicode(character) => command_string.push(character),
//                         // DecodedKey::RawKey(key) => command_string.push("{:?}", key),
//                         DecodedKey::RawKey(key) => continue,
//                     }
//                 }
//             }
//         }

//         println!("{}", command_string);
//         command_string
//     }
// }

// pub struct CommandStream {
//     _private: (), // prevent construction outside the module
// }
// impl CommandStream {
//     pub fn new() -> Self {
//         SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
//             .expect("ScancodeStream::new should only be called once");
//         ScancodeStream { _private: () }
//     }
// }
// impl Stream for pub struct CommandStream {
//     _private: (), // prevent construction outside the module
// }
// impl ScancodeStream {
//     pub fn new() -> Self {
//         SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
//             .expect("ScancodeStream::new should only be called once");
//         ScancodeStream { _private: () }
//     }
// }
// impl Stream for CommandStream {
//     type Item = u8;

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
//         let queue = SCANCODE_QUEUE
//             .try_get()
//             .expect("scancode queue not initialized");

//         // fast path
//         if let Ok(scancode) = queue.pop() {
//             return Poll::Ready(Some(scancode));
//         }

//         WAKER.register(&cx.waker());
//         match queue.pop() {
//             Ok(scancode) => {
//                 WAKER.take();
//                 Poll::Ready(Some(scancode))
//             }
//             Err(crossbeam_queue::PopError) => Poll::Pending,
//         }
//     }
// } {
//     type Item = u8;

//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
//         let queue = SCANCODE_QUEUE
//             .try_get()
//             .expect("scancode queue not initialized");

//         // fast path
//         if let Ok(scancode) = queue.pop() {
//             return Poll::Ready(Some(scancode));
//         }

//         WAKER.register(&cx.waker());
//         match queue.pop() {
//             Ok(scancode) => {
//                 WAKER.take();
//                 Poll::Ready(Some(scancode))
//             }
//             Err(crossbeam_queue::PopError) => Poll::Pending,
//         }
//     }
// }