package com.bitshift.bike_aid;

import android.icu.text.SimpleDateFormat;
import android.os.Handler;
import android.view.View;
import android.widget.ScrollView;
import android.widget.TextView;

import java.util.Date;
import java.util.Locale;

public class Logger {
    String log = "";
    TextView logView;
    ScrollView scrollView;

    public Logger(TextView view, ScrollView scroll) {
        logView = view;
        scrollView = scroll;
    }
    public void info(String message) {
        android.util.Log.d("appendLog", message);
        String strTime = new SimpleDateFormat("HH:mm:ss", Locale.getDefault()).format(new Date());
        log = log + "\n" + strTime + " " + message;
        logView.setText(log);

        // scroll after delay, because textView has to be updated first
        new Handler().postDelayed(() -> scrollView.fullScroll(View.FOCUS_DOWN), 16);
    }

    public void reset() {
        log = "";
        logView.setText(log);
    }
}
