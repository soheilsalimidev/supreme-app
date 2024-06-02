package com.app.webapp.activities

import android.annotation.SuppressLint
import android.content.Intent
import android.os.Bundle
import androidx.fragment.app.Fragment
import com.app.webapp.Config
import com.github.appintro.AppIntro
import com.github.appintro.AppIntroFragment
import com.github.appintro.AppIntroPageTransformerType

class IntroActivity : AppIntro() {

    @SuppressLint("DiscouragedApi")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        var i = 0
        Config.getInstance(this).configType.introPage.pages.forEach { item ->
            i++
            if (item != null) {
                val resId = this.resources.getIdentifier(
                    item.imageName,
                    "drawable",
                    this.packageName
                )
             val bgID =    this.resources.getIdentifier(
                    item.background,
                    "drawable",
                    this.packageName
                )
                addSlide(
                    AppIntroFragment.newInstance(
                        title = item.title,
                        description = item.description,
                        imageDrawable = resId,
                        backgroundDrawable = bgID
                    )
                )
            }
        }


        val sharedPref = getSharedPreferences("shared", MODE_PRIVATE)
        with(sharedPref.edit()) {
            putBoolean("isIntroShowed", true)
            apply()
        }

        setTransformer(
            AppIntroPageTransformerType.Parallax(
                titleParallaxFactor = 1.0,
                imageParallaxFactor = -1.0,
                descriptionParallaxFactor = 2.0
            )
        )

    }

    override fun onSkipPressed(currentFragment: Fragment?) {
        super.onSkipPressed(currentFragment)
        startActivity(Intent(this@IntroActivity, WebActivity::class.java))
    }

    override fun onDonePressed(currentFragment: Fragment?) {
        super.onDonePressed(currentFragment)
        startActivity(Intent(this@IntroActivity, WebActivity::class.java))
    }

}