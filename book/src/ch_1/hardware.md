# Hardware

Now it's time to connect the host computer with the development board.

1. Plug one end of the USB to the computer's USB port.
2. Plug the other end to the board's **esp32c6**-tagged USB port (or you may see a **USB** tag.)
3. A tiny red control LED may light up (if the board is brand new.)

> [!NOTE]
> The esp32-c6 also has an extra USB port, which is **UART** or **ch343**-tagged. **ch343** is used to communicate with the UART peripheral inside the MCU. Both ports can be used for flashing and both can be monitored.

List the USB ports with `lsusb | grep usb`. If `lsusb` is unavailable with `ls /dev/tty*usb*`. Examples:

- With `ls`

    ```bash
    ls /dev/tty*usb*
    # /dev/tty.usbmodem101
    ```

- With `lsusb`

    ```bash
    lsusb | grep JTAG
    # Bus (...) USB JTAG/serial debug unit (...)
    ```

<div class="center w320">
<a href="/assets/usb_connection.jpg">
<img alt="esp32-c6 usb port connected to usb port in laptop." src="/assets/usb_connection.jpg"/>
</a>
<p>Linking board and laptop through USB-C</p>
</div>

Now that the board is wired to our laptop, and registered by our OS, let's set up the needed software.
