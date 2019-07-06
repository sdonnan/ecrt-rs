ecrt-rs
=======

This project will eventually provide idiomatic Rust bindings for the IgH
EtherCAT master userspace library. It is currently in very early stages of
development, and just provides `bindgen` output in the `bindings` module.

License
-------

_This license was chosen to match the license of the IgH EtherCAT Master
userspace library._

IgH EtherCAT Master Rust Bindings. Copyright (C) 2019 A. Stuart Donnan

This library is free software; you can redistribute it and/or modify it under
the terms of the GNU Lesser General Public License as published by the Free
Software Foundation; either version 2.1 of the License, or (at your option) any
later version.

This library is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License along
with this library; if not, write to the Free Software Foundation, Inc., 59
Temple Place, Suite 330, Boston, MA 02111-1307 USA

EtherCAT
--------

EtherCAT technology is licensed by Beckhoff Automation. This wrapper does not
implement EtherCAT, it only provides an interface to a 3rd Party implementation
of an EtherCAT master.
