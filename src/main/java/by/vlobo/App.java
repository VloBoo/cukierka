package by.vlobo;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.ResultSet;
import java.sql.Statement;
import java.util.Properties;

public class App 
{
    public static void main(String[] args) throws Exception
    {
        Class.forName("org.postgresql.Driver");
        String url = "jdbc:postgresql://localhost/cukierka";
        Properties props = new Properties();
        props.setProperty("user", "VloBo");
        props.setProperty("password", "   1");
        Connection conn = DriverManager.getConnection(url, props);
        Statement st = conn.createStatement();
        ResultSet rs = st.executeQuery("SELECT datname FROM pg_database;");
        while (rs.next()) {
            System.out.print("Column 1 returned ");
            System.out.println(rs.getString(1));
        }
        rs.close();
        st.close();
        System.out.println("End!");
    }
}
