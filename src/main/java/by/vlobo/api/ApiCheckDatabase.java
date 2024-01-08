package by.vlobo.api;

import org.json.JSONObject;

import by.vlobo.AApiProcessor;
import by.vlobo.App;
import by.vlobo.IApiProcessor;
import by.vlobo.Database;

@AApiProcessor(key = "CheckDatabase")
public class ApiCheckDatabase implements IApiProcessor {

    @Override
    public String process(JSONObject message, App instance) {

        Database.preset();
        Database database = new Database("VloBo", " 1");
        return database.basecheck();
    }
}
