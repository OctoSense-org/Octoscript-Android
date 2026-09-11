package dev.octoscript.probe;

import java.nio.ByteBuffer;

public final class Native {
    static { System.loadLibrary("octoscript_android_probe"); }

    /** One JNI crossing: the whole UiNode tree as a direct ByteBuffer. */
    public static native ByteBuffer buildOps();

    /** Whether octoscript-render actually evaluated the DSL on device. */
    public static native String diag();
}
