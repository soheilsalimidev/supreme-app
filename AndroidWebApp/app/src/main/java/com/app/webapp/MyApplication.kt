package com.app.webapp

import android.app.Application
import com.google.android.material.color.DynamicColors

class MyApplication : Application() {
    override fun onCreate() {
        val config = Config(this);

        super.onCreate()
        if (config.configType.m3Colors)
            DynamicColors.applyToActivitiesIfAvailable(this);
    }
}