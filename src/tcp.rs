// tcp.rs

/*
 * Copyright (C) 2021 Richard Danter. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 *   * Neither the name of the author nor the names of any contributors
 *     may be used to endorse or promote products derived from this software
 *     without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

use std::net::{TcpListener, TcpStream, Ipv4Addr};
use std::io::{Read, Write};
use std::thread;

// RFC-862 specifies that the TCP Echo Service use port 7
const ECHO_PORT:u16 = 7;

// All messages start with this string
const LOG_PREFIX: &str = "echod: tcp";

pub fn listener() {
    // Binding to the UNSPECIFIED address actually binds to all interfaces
    match TcpListener::bind((Ipv4Addr::UNSPECIFIED, ECHO_PORT)) {
        Ok(socket) => {
            // We get a new stream for each connection
            for stream in socket.incoming() {
                match stream {
                    Ok(stream) => {
                        // Start a thread to handle the connection
                        thread::spawn(|| echo(stream));
                    },
                    Err(e) => {
                        eprintln!("{}: error: incoming(): {}", LOG_PREFIX, e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}: error: bind(): {}", LOG_PREFIX, e);
            return;
        }
    }

}

fn echo(mut stream: TcpStream) {
    // A buffer to store the received data
    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        // Read data from the client
        match stream.read(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    // Zero length means client closed connection
                    break;
                }

                // Write the data we read back to the client
                match stream.write(&buf[0..n]) {
                    Ok(_) => continue,
                    Err(e) => {
                        eprintln!("{}: error: write(): {}", LOG_PREFIX, e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: error: read(): {}", LOG_PREFIX, e);
                break;
            }
        }
    }
}
