//#![macro_export]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Lit, Meta, NestedMeta};

#[proc_macro]
#[proc_macro_error] // Optional: for better error handling
pub fn warn(args: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let args = parse_macro_input!(args as syn::Expr);

    // Generate the logging code
    let expanded = quote! {
        {
            // Call the original defmt::warn! macro
            defmt::warn!(#args);
            
            // Here you can add additional functionality, e.g., sending over BLE
            // For example:
            let message = format!("{:?}", #args); // Convert to string
            signals::send_ble(signals::BleHandles::Uart, message.as_bytes());
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}


#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        // Call the original defmt::warn! macro
        defmt::warn!($($arg)*);

        
        // Create a buffer to hold the formatted message
        let mut buffer = [0u8; 128]; // Adjust the size as needed
        let mut writer = &mut buffer[..];

        // Write the formatted message to the buffer
        let _ = write!(writer, $($arg)*); // Ignore the result for simplicity


        // format and write! to uart
        signals::send_ble(signals::BleHandles::Uart, writer.as_bytes());
    };
}