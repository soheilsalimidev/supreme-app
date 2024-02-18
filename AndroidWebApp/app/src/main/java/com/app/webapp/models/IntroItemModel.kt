package com.app.webapp.models

import androidx.annotation.DrawableRes
import com.app.webapp.R

class IntroItemModel(val title: String, val description: String, @DrawableRes val image: Int, private val _background: Int) {

    fun getBackground(): Int {
        return when (_background) {
            1 -> R.drawable.bg_gradient_slide1
            2 -> R.drawable.bg_gradient_slide2
            3 -> R.drawable.bg_gradient_slide3
            4 -> R.drawable.bg_gradient_slide4
            else -> R.drawable.bg_gradient_slide5
        }
    }

}