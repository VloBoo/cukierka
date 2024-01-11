package by.vlobo.api;

import org.json.JSONObject;

import by.vlobo.App;
import by.vlobo.IApiProcessor;
import by.vlobo.Database;

public class ApiCheckDatabase implements IApiProcessor {

    @Override
    public JSONObject process(JSONObject message, App instance) {

        Database.preset();
        Database database = new Database("VloBo", " 1");
        return database.basecheck();
    }
}
