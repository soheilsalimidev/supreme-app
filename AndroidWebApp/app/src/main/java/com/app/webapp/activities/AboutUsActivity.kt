package com.app.webapp.activities

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import com.app.webapp.BuildConfig
import com.app.webapp.Config
import com.app.webapp.R

class AboutUsActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_about_us)
        findViewById<TextView>(R.id.version).text = BuildConfig.VERSION_NAME
        findViewById<TextView>(R.id.aboutText).text =
            if (Config(this).configType.aboutUs.text != null) Config(this).configType.aboutUs.text else ""
    }
}