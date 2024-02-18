package com.app.webapp.activities

import android.content.Intent
import android.os.Bundle
import android.os.CountDownTimer
import android.util.Log
import androidx.appcompat.app.AppCompatActivity
import androidx.constraintlayout.motion.widget.MotionLayout
import androidx.constraintlayout.widget.ConstraintLayout
import com.app.webapp.Config
import com.app.webapp.R

class SplashActivity : AppCompatActivity() {
    private lateinit var config:Config ;
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        config = Config(this);
        val webActivityIntent = Intent(this, WebActivity::class.java)
        val sharedPref = getSharedPreferences("shared", MODE_PRIVATE)

        if (!sharedPref.getBoolean("isIntroShowed", false) && config.configType.introPage) startActivity(Intent(this@SplashActivity, IntroActivity::class.java))
        else {
            when (config.configType.splash_screen) {
                1 -> {
                    setContentView(R.layout.splash_simpel)
                    timer(webActivityIntent)
                }
                2 -> {
                    setContentView(R.layout.splash_animation)

                    val transitionListener = object : MotionLayout.TransitionListener {

                        override fun onTransitionStarted(p0: MotionLayout?, startId: Int, endId: Int) {}

                        override fun onTransitionChange(p0: MotionLayout?, startId: Int, endId: Int, progress: Float) {
                        }

                        override fun onTransitionCompleted(p0: MotionLayout?, currentId: Int) {
                            startActivity(webActivityIntent)
                            finish()
                        }

                        override fun onTransitionTrigger(p0: MotionLayout?, triggerId: Int, positive: Boolean, progress: Float) {
                        }
                    }

                    findViewById<MotionLayout>(R.id.motionLayout).setTransitionListener(transitionListener)

                }
                3 -> {
                    setContentView(R.layout.splash_animation)
                    timer(webActivityIntent)
                }
                4 -> {
                    setContentView(R.layout.splash_simpel)
                    findViewById<ConstraintLayout>(R.id.splash_simpel_root).setBackgroundResource(when (config.configType.splash_screen_g_c) {
                        1 -> R.drawable.bg_gradient_slide1
                        2 -> R.drawable.bg_gradient_slide2
                        3 -> R.drawable.bg_gradient_slide3
                        4 -> R.drawable.bg_gradient_slide4
                        else -> R.drawable.bg_gradient_slide5
                    })
                    timer(webActivityIntent)
                }
                else -> {
                    startActivity(webActivityIntent)
                    finish()
                }
            }
        }
    }

    private fun timer(intent: Intent) {
        val timer = object : CountDownTimer(5000, 1000) {
            override fun onTick(millisUntilFinished: Long) {}

            override fun onFinish() {
                startActivity(intent)
                finish()
            }
        }
        timer.start()
    }

}