package com.bitshift.bike_aid;

public class Signals {

    // ==== notes ====
    /*
    This class interfaces between the gui and the data
     */


    // ==== variables ====
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

    public void init() {

    }

    public void setSpeed(int s) {
        mOnEventListener.onSpeed(String.valueOf(s));
    }

    public void setClockMinutes(int s) {
        mOnEventListener.onClockMinutes(String.valueOf(s));
    }

    public void setClockHours(int s) {
        mOnEventListener.onClockHours(String.valueOf(s));
    }
}
