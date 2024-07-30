package com.bitshift.bike_aid;

import android.icu.text.SimpleDateFormat;
import android.os.Handler;
import android.os.Looper;

import java.util.Date;
import java.util.Locale;

public class Logger {
    // ==== notes ====
    /*
    simple log wrapper so we can display to the user
    */

    // ==== variables ====
    private static Logger mInstance = new Logger();
    String log = "";


    // ==== listener interface ====
    private OnEventListener mOnEventListener;
    public void setOnEventListener(OnEventListener listener) {
        mOnEventListener = listener;
    }

    public interface OnEventListener {
        void onUpdate(String result);
    }


    // ==== functions ====
    public static Logger getInstance() {
        return mInstance;
    }

    private Logger() {}


    public void info(String message) {
        android.util.Log.d("appendLog", message);
        String strTime = new SimpleDateFormat("HH:mm:ss", Locale.getDefault()).format(new Date());
        log = log + strTime + " " + message + "\n";
        update();
    }

    public void reset() {
        log = "";
        update();
    }


    public void update() {
        // run on main ui thread
        // we want to be able to log other threads
        new Handler(Looper.getMainLooper()).post(new Runnable() {
            public void run() {
                mOnEventListener.onUpdate(log);
            }
        });
    }

}
