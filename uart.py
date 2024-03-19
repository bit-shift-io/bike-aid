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
import tkinter as Tkinter
from tkinter import ttk
from tkinter import *
import serial


serial_data = ''
filter_data = ''
update_period = 5
serial_object = None
gui = Tk()
gui.title("UART Interface")


def connect():
    """The function initiates the Connection to the UART device with the Port and Buad fed through the Entry
    boxes in the application.

    The radio button selects the platform, as the serial object has different key phrases 
    for Linux and Windows. Some Exceptions have been made to prevent the app from crashing,
    such as blank entry fields and value errors, this is due to the state-less-ness of the 
    UART device, the device sends data at regular intervals irrespective of the master's state.

    The other Parts are self explanatory.
    """

    version_ = button_var.get()
    print (version_)
    global serial_object
    port = port_entry.get()
    baud = baud_entry.get()

    try:
        if version_ == 2:
            try:
                serial_object = serial.Serial('/dev/tty' + str(port), baud, timeout=10)
            except:
                print("Cant Open Specified Port")
        elif version_ == 1:
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
            serial_data = serial_object.readline().decode()
            if not serial_data.strip():
                continue
            #serial_data = serial_object.readline().strip('\n').strip('\r')
            #filter_data = serial_data.split(',')
            print(serial_data.strip())
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

    text.place(x = 15, y = 10)
    progress_1.place(x = 60, y = 100)
    progress_2.place(x = 60, y = 130)
    progress_3.place(x = 60, y = 160)
    progress_4.place(x = 60, y = 190)
    progress_5.place(x = 60, y = 220)
    new = time.time()
    
    while True:
        if serial_data:
            text.insert(END, serial_data)

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
    #frames
    frame_1 = Frame(height = 300, width = 480, bd = 3, relief = 'groove').place(x = 7, y = 5)
    frame_2 = Frame(height = 100, width = 480, bd = 3, relief = 'groove').place(x = 7, y = 310)
    text = Text(width = 140, height = 5) #.place(x = 5, y = 5)

    
    #threads
    t2 = threading.Thread(target = update_gui)
    t2.daemon = True
    t2.start()

    
    #Labels
    data1_ = Label(text = "Data1:").place(x = 15, y= 100)
    data2_ = Label(text = "Data2:").place(x = 15, y= 130)
    data3_ = Label(text = "Data3:").place(x = 15, y= 160)
    data4_ = Label(text = "Data4:").place(x = 15, y= 190)
    data5_ = Label(text = "Data5:").place(x = 15, y= 220)

    baud   = Label(text = "Baud").place(x = 100, y = 348)
    port   = Label(text = "Port").place(x = 200, y = 348)

    #progress_bars
    progress_1 = ttk.Progressbar(orient = HORIZONTAL, mode = 'determinate', length = 200, max = 255)
    progress_2 = ttk.Progressbar(orient = HORIZONTAL, mode = 'determinate', length = 200, max = 255)
    progress_3 = ttk.Progressbar(orient = HORIZONTAL, mode = 'determinate', length = 200, max = 255)
    progress_4 = ttk.Progressbar(orient = HORIZONTAL, mode = 'determinate', length = 200, max = 255)
    progress_5 = ttk.Progressbar(orient = HORIZONTAL, mode = 'determinate', length = 200, max = 255)



    #Entry

    # send
    data_entry = Entry()
    data_entry.place(x = 100, y = 255)
    
    baud_entry = Entry(width = 7)
    baud_entry.place(x = 100, y = 365)
    baud_entry.insert(0, '9600')
    
    port_entry = Entry(width = 7)
    port_entry.place(x = 200, y = 365)
    port_entry.insert(0, 'USB0')



    #radio button
    button_var = IntVar()
    radio_1 = Radiobutton(text = "Windows", variable = button_var, value = 1).place(x = 10, y = 315)
    radio_2 = Radiobutton(text = "Linux", variable = button_var, value = 2).place(x = 110, y = 315)
    button_var.set(2)

    #button
    button1 = Button(text = "Send", command = send, width = 6).place(x = 15, y = 250)
    connect = Button(text = "Connect", command = connect).place(x = 15, y = 360)
    disconnect = Button(text = "Disconnect", command = disconnect).place(x =300, y = 360)
   
    #mainloop
    gui.geometry('1000x500')
    gui.mainloop()
