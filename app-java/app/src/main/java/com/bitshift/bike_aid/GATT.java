package com.bitshift.bike_aid;

import android.bluetooth.BluetoothGatt;
import android.bluetooth.BluetoothGattCallback;
import android.bluetooth.BluetoothGattCharacteristic;
import android.bluetooth.BluetoothGattDescriptor;
import android.bluetooth.BluetoothGattService;
import android.os.Handler;

import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

public class GATT extends BluetoothGattCallback {

    // ==== notes ====
    /*
    This class handles the read and write of data
     */


    // ==== variables ====
    protected static final UUID CHARACTERISTIC_UPDATE_NOTIFICATION_DESCRIPTOR_UUID = UUID.fromString("00002902-0000-1000-8000-00805f9b34fb");
    private static final Logger log = Logger.getInstance();
    private static final Signals signals = Signals.getInstance();
    private static BluetoothGatt mGatt;
    ArrayList<BluetoothGattCharacteristic> processedCharacteristics = new ArrayList<>();


    // ==== listener interface ====
    private OnEventListener mOnEventListener;
    public void setOnEventListener(OnEventListener listener) {
        mOnEventListener = listener;
    }
    public interface OnEventListener {
        void onUpdate(String result);
    }


    // ==== functions ====
    public GATT () {};


    // setup notifications
    public boolean setCharacteristicNotification(BluetoothGatt gatt, UUID serviceUuid, UUID characteristicUuid, boolean enable) {
        // get characteristic
        BluetoothGattCharacteristic characteristic = gatt.getService(serviceUuid).getCharacteristic(characteristicUuid);

        // set characteristic notification
        gatt.setCharacteristicNotification(characteristic, enable);

        // write to descriptor for notification
        BluetoothGattDescriptor descriptor = characteristic.getDescriptor(CHARACTERISTIC_UPDATE_NOTIFICATION_DESCRIPTOR_UUID);
        if (descriptor == null)
            return false;

        byte[] val = enable ? BluetoothGattDescriptor.ENABLE_NOTIFICATION_VALUE : BluetoothGattDescriptor.DISABLE_NOTIFICATION_VALUE;
        //descriptor.setValue(val);
        int result = gatt.writeDescriptor(descriptor, val); //descriptor write operation successfully started?
        return true;
    }


    // this function will process the gatt characteristics descriptor
    // there is a delay with BLE so we need to wait till we receive the ok
    // then process the next in the list
    public void processNextCharacteristic(BluetoothGatt gatt) {
        // get list of all services
        List<BluetoothGattService> services = gatt.getServices();

        // loop services
        for (BluetoothGattService s : services) {
            List<BluetoothGattCharacteristic> characteristics = s.getCharacteristics();

            // loop characteristics
            for (BluetoothGattCharacteristic c : characteristics) {

                // have we already been configured?
                if (processedCharacteristics.contains(c))
                        continue;

                // set notification
                boolean hasNotify = setCharacteristicNotification(
                        gatt,
                        s.getUuid(),  // service
                        c.getUuid(),  // characteristic
                        true);

                // add to completed list
                processedCharacteristics.add(c);

                // has notify, so wait for response
                // if not try another characteristic
                if (hasNotify)
                    return;
            }
        }
    }


    // ==== overrides ====
    public void onCharacteristicWrite (BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, int status) {
        log.info("WRITE!");

    }


    @Override
    public void onConnectionStateChange(BluetoothGatt gatt, int status, int newState) {
        signals.setGatt(gatt);
        boolean connected = BluetoothGatt.GATT_SUCCESS == status;
        log.info("BLE connected to device: " + String.valueOf(connected));
        gatt.discoverServices();
    }


    @Override
    public void onCharacteristicChanged(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value) {
        gatt.readCharacteristic(characteristic);
    }

    @Override
    public void onCharacteristicRead(BluetoothGatt gatt, BluetoothGattCharacteristic characteristic, byte[] value, int status) {
        String id = Functions.string16FromUUID(characteristic.getUuid());

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
        // clock minutes
        if (id.equals("2005")) {
            signals.setClockMinutes(value[0]);
        }

        // clock hours
        if (id.equals("2006")) {
            signals.setClockHours(value[0]);
        }


    }

    @Override
    public void onDescriptorWrite (BluetoothGatt gatt, BluetoothGattDescriptor descriptor, int status) {
        String s = Functions.string16FromUUID(descriptor.getCharacteristic().getUuid());
        log.info("BLE notify: " + s);
        processNextCharacteristic(gatt);
    }

    @Override
    public void onServicesDiscovered(BluetoothGatt gatt, int status) {
        log.info("BLE service discovery...");

        // start the process of configuring descriptors
        processNextCharacteristic(gatt);
    }

}
