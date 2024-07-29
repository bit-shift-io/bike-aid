package com.bitshift.bike_aid;

import android.icu.text.SimpleDateFormat;
import android.os.Handler;
import android.view.View;
import android.widget.ScrollView;
import android.widget.TextView;

import java.util.Date;
import java.util.Locale;

public class Logger {
    private static Logger ourInstance = new Logger();
    String log = "";

    private OnEventListener mOnEventListener;
    public void setOnEventListener(OnEventListener listener) {
        mOnEventListener = listener;
    }

    public interface OnEventListener {
        void onUpdate(String result);
    }
    public static Logger getInstance() {
        return ourInstance;
    }
    private Logger() {
    }
    public void info(String message) {
        android.util.Log.d("appendLog", message);
        String strTime = new SimpleDateFormat("HH:mm:ss", Locale.getDefault()).format(new Date());
        log = log + "\n" + strTime + " " + message;
        mOnEventListener.onUpdate(log);
    }

    public void reset() {
        log = "";
        mOnEventListener.onUpdate(log);
    }
}
