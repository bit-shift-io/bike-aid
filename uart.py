"""
This is a generic application for sending and receiving data from the computer to UART host controller (Arduino).

The major functions are self update and get data which are threaded to make sure the GUI does not freeze.
The GUI runs in the main thread, the worker threads are the two separate ones.

Based on https://github.com/pratikguru/Instructables
Updated for python 3

Install tk:
sudo pacman -S tk python-pyserial

Note: this uses readline command, so make sure your data ends with /n from the arduino!
"""


import time
import threading
#import tkinter as tk
#from tkinter import ttk
from tkinter import *
import serial


serial_data = ''
filter_data = ''

update_period = 5
serial_object = None
gui = Tk()
gui.title("UART")
platform = StringVar(gui)


def connect():
    """The function initiates the Connection to the UART device with the Port and Buad fed through the Entry
    boxes in the application.

    The radio button selects the platform, as the serial object has different key phrases 
    for Linux and Windows. Some Exceptions have been made to prevent the app from crashing,
    such as blank entry fields and value errors, this is due to the state-less-ness of the 
    UART device, the device sends data at regular intervals irrespective of the master's state.

    The other Parts are self explanatory.
    """

    global serial_object
    port = port_entry.get()
    baud = baud_entry.get()
    p = platform.get()

    try:
        if p == 'Linux':
            try:
                serial_object = serial.Serial('/dev/tty' + str(port), baud, timeout=10)
            except:
                print("Cant Open Specified Port")
        elif p == 'Windows':
            serial_object = serial.Serial('COM' + str(port), baud, timeout=10)

    except ValueError:
        print("Enter Baud and Port")
        return

    print("connected: ", serial_object.isOpen())
    t1 = threading.Thread(target = get_data)
    t1.daemon = True
    t1.start()
    


def get_data():
    """This function serves the purpose of collecting data from the serial object and storing 
    the filtered data into a global variable.

    The function has been put into a thread since the serial event is a blocking function.
    """
    global serial_object
    global filter_data
    global serial_data

    while True:
        try:
            serial_data = serial_object.readline().decode('utf-8', 'ignore')
            if not serial_data.strip():
                continue
            #serial_data = serial_object.readline().strip('\n').strip('\r')
            #filter_data = serial_data.split(',')
            #print(serial_data.strip())
        except TypeError:
            pass
    
    
        

def update_gui():
    """" This function is an update function which is also threaded. The function assimilates the data
    and applies it to it corresponding progress bar. The text box is also updated every couple of seconds.

    A simple auto refresh function .after() could have been used, this has been avoid purposely due to 
    various performance issues.


    """
    global filter_data
    global update_period
    global serial_data

    new = time.time()

    while True:
        if serial_data:
            #serial_text = gui.nametowidget("serial_frame.serial_text")
            serial_text.insert(END, serial_data)
            serial_data = ''
            serial_text.see(END)

        if filter_data:
            try:
                progress_1["value"] = filter_data[0]
                progress_2["value"] = filter_data[1]
                progress_3["value"] = filter_data[2]
                progress_4["value"] = filter_data[3]
                progress_5["value"] = filter_data[4]

            
            except :
                pass

            
            if time.time() - new >= update_period:
                text.delete("1.0", END)
                progress_1["value"] = 0
                progress_2["value"] = 0
                progress_3["value"] = 0
                progress_4["value"] = 0
                progress_5["value"] = 0
                new = time.time()


def send():
    """This function is for sending data from the computer to the host controller.
    

        The value entered in the the entry box is pushed to the UART. The data can be of any format, since
        the data is always converted into ASCII, the receiving device has to convert the data into the required f
        format.
    """
    send_data = data_entry.get()
    
    if not send_data:
        print("Sent Nothing")
    
    serial_object.write(send_data)



def disconnect():
    """ 
    This function is for disconnecting and quitting the application.

    Sometimes the application throws a couple of errors while it is being shut down, the fix isn't out yet
    but will be pushed to the repo once done.

    simple GUI.quit() calls.

    """
    try:
        serial_object.close() 
    
    except AttributeError:
        print("Closed without Using it -_-")

    gui.quit()



if __name__ == "__main__":

    """
    The main loop consists of all the GUI objects and its placement.

    The Main loop handles all the widget placements.

    """

    # connect frame
    connect_frame = Frame(gui)
    connect_frame.pack(pady=10)

    port_label = Label(connect_frame, text="Port:")
    port_label.grid(row=0, column=0, sticky="W", padx = (10, 5))
    port_entry = Entry(connect_frame)
    port_entry.grid(row=0, column=1, padx = (5, 10))
    port_entry.insert(0, 'USB0')

    baud_label = Label(connect_frame, text="Baud:")
    baud_label.grid(row=0, column=2, sticky="W", padx = (10, 5))
    baud_entry = Entry(connect_frame)
    baud_entry.grid(row=0, column=3, padx = (5, 10))
    baud_entry.insert(0, '9600')

    platform_options = ['Linux','Windows','OSX']
    platform.set('Linux') # set the default option
    drop_platform = OptionMenu(connect_frame, platform, *platform_options).grid(row=0, column=4, padx = (5, 10))

    connect_button = Button(connect_frame, text = "Connect", command = connect)
    connect_button.grid(row=0, column=5, padx = (5, 10))
    disconnect_button = Button(connect_frame, text = "Disconnect", command = disconnect)
    disconnect_button.grid(row=0, column=6, padx = (5, 10))

    # serial interface
    serial_frame = Frame(gui, name = "serial_frame")
    serial_frame.pack(fill="x", expand=True, pady=10)
    scroll = Scrollbar(serial_frame)
    scroll.pack(side=RIGHT, fill=Y)
    serial_text = Text(serial_frame, name = "serial_text", yscrollcommand=scroll.set, height=20)
    serial_text.pack(fill='x')
    print(serial_text)
    scroll.config(command=serial_text.yview)



    # automated interface
    data_frame = Frame(gui)
    data_frame.pack(fill="both", expand=True, pady=10)

    data_template_label = Label(data_frame, text="Data: 1.04").grid(row=0, column=0, sticky="W", padx = (10, 5))

    # threads
    t2 = threading.Thread(target = update_gui)
    t2.daemon = True
    t2.start()

    gui.mainloop()