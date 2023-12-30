package by.vlobo.handlers;

import com.sun.net.httpserver.HttpHandler;
import com.sun.net.httpserver.HttpExchange;

import java.io.File;
import java.io.IOException;
import java.io.OutputStream;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;

public class CdnHandler implements HttpHandler {
    @Override
    public void handle(HttpExchange t) throws IOException {
        String path = t.getRequestURI().getPath();

        String filePath = "." + path; // TODO: пока что из корня проекта и из папки cdn, надо исправить
        System.out.println(filePath);

        File file = new File(filePath);

        if (file.isDirectory()) {
            List<String> fileNames = new ArrayList<>();
            for (File f : file.listFiles()) {
                fileNames.add(f.getName());
            }

            String response = String.join("\n", fileNames);
            t.sendResponseHeaders(200, response.length());
            try (OutputStream os = t.getResponseBody()) {
                os.write(response.getBytes());
            }
        } else if (file.exists() && file.isFile()) {
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
