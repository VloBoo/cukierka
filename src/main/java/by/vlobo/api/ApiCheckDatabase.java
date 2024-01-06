package by.vlobo.api;

import by.vlobo.ApiClass;
import by.vlobo.ApiProcessor;
import by.vlobo.Database;

@ApiClass(key = "CheckDatabase")
public class ApiCheckDatabase implements ApiProcessor {

    @Override
    public String process(String message) {

        Database.preset();
        Database database = new Database("VloBo", " 1");
        return database.basecheck();
    }
}
