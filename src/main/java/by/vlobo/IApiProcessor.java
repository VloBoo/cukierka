package by.vlobo;

import org.json.JSONObject;

public interface IApiProcessor {
    JSONObject process(JSONObject message, App instance);
}
