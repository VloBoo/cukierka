package by.vlobo;

import com.sun.net.httpserver.HttpServer;

import by.vlobo.handlers.ApiHandler;
import by.vlobo.handlers.CdnHandler;
import by.vlobo.handlers.WwwHandler;

import java.io.IOException;
import java.net.InetSocketAddress;

public class Server {
    public Server() {
        try {
            HttpServer server = HttpServer.create(new InetSocketAddress(8080), 0);
            server.createContext("/api", new ApiHandler());
            server.createContext("/cdn", new CdnHandler("."));
            server.createContext("/", new WwwHandler());
            server.setExecutor(null);
            server.start();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}