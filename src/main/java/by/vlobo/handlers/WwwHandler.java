package by.vlobo.handlers;

import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpExchange;

import java.io.IOException;
import java.io.OutputStream;


public class WwwHandler implements HttpHandler {
    @Override
    public void handle(HttpExchange t) throws IOException {
        String response = "hello, it is my first main page";
        t.sendResponseHeaders(200, response.length());
        try (OutputStream os = t.getResponseBody()) {
            os.write(response.getBytes());
        }
    }
}