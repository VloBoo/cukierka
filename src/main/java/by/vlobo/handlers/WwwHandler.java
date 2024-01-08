package by.vlobo.handlers;

import com.sun.net.httpserver.HttpHandler;

import by.vlobo.App;

import com.sun.net.httpserver.HttpExchange;

import java.io.File;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.file.Files;

public class WwwHandler implements HttpHandler {

    private App app;

    public WwwHandler(App app) {
        this.app = app;
    }

    @Override
    public void handle(HttpExchange t) throws IOException {
        File file = new File("www/index.html");
        if (file.exists() && file.isFile()) {
            // не всегда будет html, пока заглушка ради кодировки
            t.getResponseHeaders().set("Content-Type", "text/html; charset=UTF-8");
            t.sendResponseHeaders(200, file.length());
            try (OutputStream os = t.getResponseBody()) {
                Files.copy(file.toPath(), os);
            }
        } else {
            String notFoundResponse = "File not found";
            t.sendResponseHeaders(404, notFoundResponse.length());
            try (OutputStream os = t.getResponseBody()) {
                os.write(notFoundResponse.getBytes());
            }
        }
    }
}