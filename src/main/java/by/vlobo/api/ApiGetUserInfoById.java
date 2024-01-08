package by.vlobo.api;

import org.json.JSONObject;

import by.vlobo.AApiProcessor;
import by.vlobo.IApiProcessor;
import by.vlobo.App;

@AApiProcessor(key = "CheckDatabase")
public class ApiGetUserInfoById implements IApiProcessor {

    @Override
    public String process(JSONObject message, App instance) {

        return "{}";
    }
}