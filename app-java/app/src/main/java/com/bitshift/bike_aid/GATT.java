package com.bitshift.bike_aid;

public class GATT {

    // ==== notes ====
    /*
    This class handles the read and write of data
     */


    // ==== variables ====
    private static GATT mInstance = new GATT();

    public static GATT getInstance() {
        return mInstance;
    }


    // ==== listener interface ====
    private OnEventListener mOnEventListener;
    public void setOnEventListener(OnEventListener listener) {
        mOnEventListener = listener;
    }
    public interface OnEventListener {
        void onUpdate(String result);
    }


    // ==== functions ====
    private GATT () {};

    public void init() {

    }
}
