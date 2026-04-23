package com.example.waitimcooking

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform