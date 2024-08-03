package com.bitshift.bike_aid;

import java.util.UUID;

public class Functions {

    // Converts 16 bit UUIDs to 128 bit format
    // the 16 bit uuid is short for 0000xxxx-0000-1000-8000-00805F9B34FB
    public static UUID uuidFrom16(String uuid16) {
        String baseUUIDSuffix = "0000-1000-8000-00805F9B34FB";
        String uuid = "0000" + uuid16 + baseUUIDSuffix;
        return UUID.fromString(uuid);
    }


    // Converts 128 bit UUIDs to 16 bit string
    // the 16 bit uuid is short for 0000xxxx-0000-1000-8000-00805F9B34FB
    public static String string16FromUUID(UUID uuid) {
        String s = uuid.toString();
        return s.substring(4, 8);
    }

}
