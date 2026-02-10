package com.example.vulkanoapp;

import android.os.Bundle;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;

import com.example.vulkanoapp.jni.VulkanoLab;

public class MainActivity extends AppCompatActivity {

    private final static String SO_NAME = "vulkano_lab";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        System.loadLibrary(SO_NAME);
        updateView();
    }

    void updateView(){
        TextView textView = this.findViewById(R.id.main_title);
        textView.setText(VulkanoLab.helloVulkano());

        TextView content = findViewById(R.id.main_content);
        content.setText(VulkanoLab.vulkanoInfo());
    }

}