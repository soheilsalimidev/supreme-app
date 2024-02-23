package com.app.webapp.activities

import android.content.Intent
import android.graphics.drawable.GradientDrawable
import android.os.Bundle
import android.os.CountDownTimer
import android.util.Log
import android.view.View
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

        if (!sharedPref.getBoolean("isIntroShowed", false) && config.configType.introPage.enable) startActivity(Intent(this@SplashActivity, IntroActivity::class.java))
        else {
            when (config.configType.splashScreen.type) {
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
                    findViewById<ConstraintLayout>(R.id.splash_simpel_root).setBackgroundResource(R.drawable.bg_gradient_splash)
                    timer(webActivityIntent)
                }
                else -> {
                    startActivity(webActivityIntent)
                    finish()
                }
            }
        }
    }

 //   fun View.gradient(radius: Float, vararg colors: Int) {
        //"radial-gradient(circle, rgba(235, 65, 101, 1) 0%, rgba(207, 147, 217, 1) 99%)"
        //"linear-gradient(106deg, rgba(235, 65, 101, 1) 0%, rgba(207, 147, 217, 1) 99%)"
     //   background = GradientDrawable().apply { cornerRadius = radius ; colors = [] }
 //   }
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