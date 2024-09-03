package com.bitshift.bike_aid;

import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCharacteristic;
import android.os.Handler;
import android.os.Looper;

import java.math.BigInteger;
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
    public boolean alarm_on = false;
    public boolean power_on = false;


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
        void onPower(boolean result);
        void onAlarm(boolean result);
        void onBatteryLevel(String result);
    }


    // ==== functions ====
    private Signals () {
        ble.setOnEventListener(this::onRead);
    };


    // ==== data -> gui ====
    public void setPower(boolean v) {
        new Handler(Looper.getMainLooper()).post(new Runnable() { // run on ui thread
            public void run() { mOnEventListener.onPower(v); }
        });
    }

    public void setAlarm(boolean v) {
        new Handler(Looper.getMainLooper()).post(new Runnable() { // run on ui thread
            public void run() { mOnEventListener.onAlarm(v); }
        });
    }

    public void setSpeed(int v) {
        new Handler(Looper.getMainLooper()).post(new Runnable() { // run on ui thread
            public void run() { mOnEventListener.onSpeed(String.format("%02d", v)); }
        });
    }


    public void setBatteryLevel(int v) {
        new Handler(Looper.getMainLooper()).post(new Runnable() { // run on ui thread
            public void run() {
                mOnEventListener.onBatteryLevel(String.valueOf(v));
            }
        });
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
    public void togglePower() {
        UUID service_id = Functions.uuidFrom16("1000");
        UUID characteristic_id = Functions.uuidFrom16("1001");
        byte[] b;
        if (power_on)
            b = new byte[]{ (byte) 0 };
        else
            b = new byte[]{ (byte) 183 };

        ble.write(service_id, characteristic_id, b);
    }

    public void toggleAlarm() {
        UUID service_id = Functions.uuidFrom16("1000");
        UUID characteristic_id = Functions.uuidFrom16("1004");
        byte[] b;
        if (alarm_on)
            b = new byte[]{ (byte) 0 };
        else
            b = new byte[]{ (byte) 205 };

        ble.write(service_id, characteristic_id, b);
    }

    public void setUART(String s) {
        log.info("UART Write: " + s);
        UUID uart_service = UUID.fromString("6E400001-B5A3-F393-E0A9-E50E24DCCA9E");
        UUID uart_write_characteristic = UUID.fromString("6E400002-B5A3-F393-E0A9-E50E24DCCA9E");
        byte[] b = s.getBytes(StandardCharsets.UTF_8);
        ble.write(uart_service, uart_write_characteristic, b);
    };


    // ==== on ble read ====
    public void onRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
        String id = Functions.string16FromUUID(characteristic.getUuid());
        String st = new String(value, StandardCharsets.UTF_8);

        // debug read
        log.info("read " + id + " " + st);

        // uart

        // rx - 6E400002-B5A3-F393-E0A9-E50E24DCCA9E
        // is never read, write only

        // tx - 6E400003-B5A3-F393-E0A9-E50E24DCCA9E
        if (id.equals("0003")) {
            String s = new String(value, StandardCharsets.UTF_8);
            log.info("UART Read: " + s);
        }


        // 1000 series is settings

        // power switch
        if (id.equals("1001")) {
            power_on = value[0] != 0;
            setPower(power_on);
        }

        // alarm switch
        if (id.equals("1004")) {
            alarm_on = value[0] != 0;
            setAlarm(alarm_on);
        }



        // 2000 series is data

        // speed
        if (id.equals("2001")) {
            setSpeed(value[0]);
        }

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


        // 0x180f series is battery

        // level - 0x2a19
        if (id.equals("2a19")) {
            setBatteryLevel(value[0]);
        }


    }


}
