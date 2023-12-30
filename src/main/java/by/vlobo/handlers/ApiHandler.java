package by.vlobo.handlers;

import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpExchange;

import java.io.IOException;
import java.io.OutputStream;

import by.vlobo.Database;

public class ApiHandler implements HttpHandler {
    @Override
    public void handle(HttpExchange t) throws IOException {
        Database.preset();
        Database database = new Database("VloBo", " 1");
        String response = database.basecheck();
        t.sendResponseHeaders(200, response.length());
        try (OutputStream os = t.getResponseBody()) {
            os.write(response.getBytes());
        }
    }
}
