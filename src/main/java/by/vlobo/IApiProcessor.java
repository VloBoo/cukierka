package by.vlobo;

import org.json.JSONObject;

public interface IApiProcessor{
    String process(JSONObject message, App instance);
}