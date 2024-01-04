package by.vlobo;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.Properties;

import org.json.JSONArray;
import org.json.JSONObject;

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

    public Database(String user, String password, String host, String dbName) {
        try {
            String url = "jdbc:postgresql://" + host + "/" + dbName;
            Properties props = new Properties();
            props.setProperty("user", user);
            props.setProperty("password", password);
            connection = DriverManager.getConnection(url, props);
        } catch (SQLException e) {
            e.printStackTrace();
        }
    }

    public Database(String user, String password) {
        this(user, password, "localhost", "cukierka");
    }

    public String basechec2k() {
        try {
            Statement st = connection.createStatement();
            ResultSet rs = st.executeQuery("SELECT datname FROM pg_database;");
            ArrayList<String> columns = new ArrayList<>();
            while (rs.next()) {
                columns.add(rs.getString(1));
            }
            rs.close();
            st.close();
            return new JSONObject().put("columns", columns.toArray()).toString();
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }

    public String basecheck() {
        try {
            Statement st = connection.createStatement();
            ResultSet rs = st.executeQuery("SELECT \n" + //
                    "    table_name as \"Table\",\n" + //
                    "    column_name as \"Column\"\n" + //
                    "FROM \n" + //
                    "    information_schema.columns\n" + //
                    "WHERE \n" + //
                    "    table_catalog = 'cukierka'\n" + //
                    "    AND table_schema NOT LIKE 'pg_%'\n" + //
                    "    AND table_schema != 'information_schema'\n" + //
                    "ORDER BY \n" + //
                    "    table_name, ordinal_position;\n" + //
                    "");
            JSONArray jsonArray = new JSONArray();

            while (rs.next()) {
                int columns = rs.getMetaData().getColumnCount();
                JSONObject jsonObject = new JSONObject();

                for (int i = 1; i <= columns; i++) {
                    String columnName = rs.getMetaData().getColumnName(i);
                    Object value = rs.getObject(i);
                    jsonObject.put(columnName, value);
                }

                jsonArray.put(jsonObject);
            }
            return jsonArray.toString(4);
        } catch (SQLException e) {
            e.printStackTrace();
            return null;
        }
    }
}
