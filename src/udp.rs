// udp.rs

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

use std::net::{UdpSocket, Ipv4Addr};

// RFC-862 specifies that the UDP Echo Service use port 7
const ECHO_PORT:u16 = 7;

// All messages start with this string
const LOG_PREFIX: &str = "echod: udp";

pub fn listener() {
    // Binding to the UNSPECIFIED address actually binds to all interfaces
    match UdpSocket::bind((Ipv4Addr::UNSPECIFIED, ECHO_PORT)) {
        Ok(socket) => {
            loop {
                // The socket is re-used for all datagrams
                echo(&socket);
            }
        }
        Err(e) => {
            eprintln!("{}: error: bind(): {}", LOG_PREFIX, e);
            return;
        }
    }
}

fn echo(socket: &UdpSocket) {
    // A buffer to store the received data
    let mut buf: [u8; 1024] = [0; 1024];

    // Read data from the client
    // NOTE: If the datagram is larger than the buffer then it is truncated
    match socket.recv_from(&mut buf) {
        Ok((n, client)) => {
            // Write the data we read back to the client
            match socket.send_to(&buf[0..n], client) {
                Ok(_) => return,
                Err(e) => {
                    eprintln!("{}: error: send_to(): {}", LOG_PREFIX, e);
                }
            }
        }
        Err(e) => {
            eprintln!("{}: error: recv_from(): {}", LOG_PREFIX, e);
        }
    }
}
