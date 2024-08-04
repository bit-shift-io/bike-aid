package com.bitshift.bike_aid;

import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCallback;
import android.bluetooth.BluetoothGattCharacteristic;
import android.os.Handler;
import android.os.Looper;

import java.nio.charset.StandardCharsets;
import java.util.UUID;

public class Signals {

    // ==== notes ====
    /*
    This class interfaces between the gui and the data
    */


    // ==== variables ====
    private static final Logger log = Logger.getInstance();
    private static final BLE ble = BLE.getInstance();
    private static Signals mInstance = new Signals();
    public static Signals getInstance() {
        return mInstance;
    }


    // ==== listener interface ====
    private OnEventListener mOnEventListener;
    public void setOnEventListener(OnEventListener listener) {
        mOnEventListener = listener;
    }
    public interface OnEventListener {
        void onSpeed(String result);
        void onClockMinutes(String result);
        void onClockHours(String result);
    }


    // ==== functions ====
    private Signals () {};


    // ==== data -> gui ====
    public void setSpeed(int s) {
        mOnEventListener.onSpeed(String.valueOf(s));
    }

    public void setClockMinutes(int s) {
        new Handler(Looper.getMainLooper()).post(new Runnable() { // run on ui thread
            public void run() {
                mOnEventListener.onClockMinutes(String.format("%02d", s));
            }
        });
    }

    public void setClockHours(int s) {
        new Handler(Looper.getMainLooper()).post(new Runnable() { // run on ui thread
            public void run() {
                mOnEventListener.onClockHours(String.format("%02d", s));
            }
        });
    }

    // ==== gui -> data ====
    public void setPower(int v) { log.info("Signal: Power"); }

    public void setAlarm(int v) { log.info("Signal: alarm"); }

    public void setUART(String s) {
        log.info("BLE: UART: " + s);
        UUID uart_service = UUID.fromString("6E400001-B5A3-F393-E0A9-E50E24DCCA9E");
        UUID uart_write_characteristic = UUID.fromString("6E400002-B5A3-F393-E0A9-E50E24DCCA9E");
        byte[] b = s.getBytes(StandardCharsets.UTF_8);
        ble.write(uart_service, uart_write_characteristic, b);
    };


}
