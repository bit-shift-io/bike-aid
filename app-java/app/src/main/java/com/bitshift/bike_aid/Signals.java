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
    private final BLE ble = BLE.getInstance();
    private static final Signals mInstance = new Signals();
    public static Signals getInstance() {
        return mInstance;
    }


    // ==== listener interface ====
    private OnEventListener mOnEventListener;
    public void setOnEventListener(OnEventListener listener) {
        mOnEventListener = listener;
    }
    public interface OnEventListener {
        void onTemperature(String result);
        void onSpeed(String result);
        void onClockMinutes(String result);
        void onClockHours(String result);
    }


    // ==== functions ====
    private Signals () {
        ble.setOnEventListener(this::onRead);
    };


    // ==== data -> gui ====
    public void setSpeed(int s) {
        mOnEventListener.onSpeed(String.valueOf(s));
    }

    public void setTemperature(int v) {
        new Handler(Looper.getMainLooper()).post(new Runnable() { // run on ui thread
            public void run() {
                mOnEventListener.onTemperature(String.format("%02d", v));
            }
        });
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


    // ==== on ble read ====
    public void onRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
        String id = Functions.string16FromUUID(characteristic.getUuid());
        log.info("read " + id);
        // uart

        // rx - 6E400002-B5A3-F393-E0A9-E50E24DCCA9E
        if (id.equals("0002")) {
            String s = new String(value, StandardCharsets.UTF_8);
            log.info("BLE RX: " + s);
        }
        // tx - 6E400003-B5A3-F393-E0A9-E50E24DCCA9E
        if (id.equals("0003")) {
            String s = new String(value, StandardCharsets.UTF_8);
            log.info("BLE: TX: " + s);
        }


        // 1000 series is settings


        // 2000 series is data

        // temperature
        if (id.equals("2004")) {
            setTemperature(value[0]);
        }

        // clock minutes
        if (id.equals("2005")) {
            setClockMinutes(value[0]);
        }

        // clock hours
        if (id.equals("2006")) {
            setClockHours(value[0]);
        }


    }


}
