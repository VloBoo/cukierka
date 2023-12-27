package by.vlobo;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;
import java.util.Properties;

// Этот класс должен служить прослойкой между бэком, что бы я спокойно вносил правки в бд и тут, а само приложение не трогал.
public class Database {
    private Connection connection;

    public static void preset() {
        try {
            Class.forName("org.postgresql.Driver");
        } catch (ClassNotFoundException e) {
            System.err.println("Dont found class name");
            e.printStackTrace();
        }
    }

    public Database(String user, String password) throws Exception {
        String url = "jdbc:postgresql://localhost/cukierka";
        Properties props = new Properties();
        props.setProperty("user", user);
        props.setProperty("password", password);
        connection = DriverManager.getConnection(url, props);
    }

    public String basecheck() {
        try {
            StringBuffer stringBuffer = new StringBuffer();
            Statement st = connection.createStatement();
            ResultSet rs = st.executeQuery("SELECT datname FROM pg_database;");

            while (rs.next()) {
                System.out.print("Column 1 returned ");
                System.out.println(rs.getString(1));
            }
            rs.close();
            st.close();
            return stringBuffer.toString();
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }
}
