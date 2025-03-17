// import couchbase from "couchbase";

// import dotenv from 'dotenv';
// dotenv.config();

// async function testConnection() {
//   try {
//     const clusterConnStr =
//       "couchbases://cb.mij21cud055ga4ut.cloud.couchbase.com";
//     const username = "jay";
//     const password = "Jay$2833";

//     const cluster = await couchbase.connect(clusterConnStr, {
//       username: username,
//       password: password,
//       // Use the pre-configured profile below to avoid latency issues with your connection.
//       configProfile: "wanDevelopment",
//     });

//     console.log("Couchbase Connected Successfully!");
//   } catch (error) {
//     console.error("Couchbase Connection Failed:", error);
//   }
// }

// testConnection();
async function connectAndInsert() {
  const clusterConnStr = "couchbases://cb.mij21cud055ga4ut.cloud.couchbase.com";
  const username = "jay";
  const password = "Jay$2833";
  //   try {

  //     const cluster = await couchbase.connect(clusterConnStr, {
  //       username: username,
  //       password: password,
  //       // Use the pre-configured profile below to avoid latency issues with your connection.
  //       configProfile: "wanDevelopment",
  //     });

  //     console.log("Couchbase Connected Successfully!");
  //   } catch (error) {
  //     console.error("Couchbase Connection Failed:", error);
  //   }
  try {
    console.log("⏳ Connecting to Couchbase Capella...");

    const cluster = await connect(clusterConnStr, {
      username: username,
      password: password,
      connectionOptions: {
        // Enable SSL for secure connection (only necessary for couchbases:// URL)
        secure: true,
        trustCertificate: true, // Trust the server's SSL certificate
      },
      timeoutOptions: { connectTimeout: 60000 }, // Increase connection timeout to 60 seconds
    });

    // Define the Couchbase bucket and collection
    const bucketName = "testghirkins";
    const bucket = cluster.bucket(bucketName);
    const collection = bucket.scope("_default").collection("_default");
  } catch (error) {
    console.error("❌ Operation failed:", error.message);
    process.exit(1);
  }
}

// Call the function to connect and insert the data
connectAndInsert();

// const insertReport = async (requsername) => {
//   // Define the file path for the report JSON
//   const jsonfilePath = "./Reports/cucumber-report.json"; // Replace with your actual file path
//   const htmlFilePath = "./Reports/cucumber-report.html"; // Replace with your actual file path
//   const xmlFilePath = "./Reports/cucumber-report.xml";

//   fs.readFile(jsonfilePath, "utf8", async (err, data) => {
//     if (err) {
//       console.error("Error reading the file:", err);
//       return;
//     }

//     // Parse the JSON data
//     const reportData = JSON.parse(data);

//     // Create a unique document ID
//     const documentId = requsername + "_report_json"; // Example ID

//     try {
//       // Insert the parsed data into Couchbase
//       const result = await collection.upsert(documentId, reportData);
//       console.log("✅ Data inserted successfully:", result);
//     } catch (err) {
//       console.error("❌ Error inserting data into Couchbase:", err);
//     }
//   });

//   fs.readFile(htmlFilePath, "utf8", async (err, htmlData) => {
//     if (err) {
//       console.error("Error reading HTML file:", err);
//       return;
//     }

//     const documentId = requsername + "_report_html"; // Unique ID for HTML report

//     try {
//       const result = await collection.upsert(documentId, { html: htmlData });
//       console.log("✅ HTML Data inserted successfully:", result);
//     } catch (err) {
//       console.error("❌ Error inserting HTML data into Couchbase:", err);
//     }
//   });

//   fs.readFile(xmlFilePath, "utf8", async (err, xmlData) => {
//     if (err) {
//       console.error("Error reading HTML file:", err);
//       return;
//     }

//     const documentId = requsername + "_report_xml"; // Unique ID for HTML report

//     try {
//       const result = await collection.upsert(documentId, { xml: xmlData });
//       console.log("✅ XML Data inserted successfully:", result);
//     } catch (err) {
//       console.error("❌ Error inserting XML data into Couchbase:", err);
//     }
//   });
// };

// app.get("/", (req, res) => {
//   res.sendFile(__dirname + "/public/index.html");
// });

// app.get("/username", (req, res) => {
//   // Extract username from the request's header or query
//   user = req.get("username");
//   res.send(`Hello, ${user}! Your username is saved in the request.`);
// });

// app.get("/run-test", async (req, res) => {
//   try {
//     const requsername = req.headers.username; // Get the username from request headers
//     if (!username) {
//       return res.status(400).json({ message: "Username is required" });
//     }
//     console.log(requsername);
//     // Run Cucumber tests
//     const { stdout: cucumberOutput, stderr: cucumberError } = await execPromise(
//       "npm run test"
//     );
//     // console.log("Cucumber Test Output:", cucumberOutput);

//     if (cucumberError) {
//       console.error(`Cucumber Test Stderr: ${cucumberError}`);
//     }
//     await insertReport(requsername);
//     return res.json({
//       message: "Tests executed successfully with report",
//     });
//   } catch (error) {
//     console.error(`Error: ${error.message}`);
//     return res
//       .status(500)
//       .json({ message: "Test execution failed", error: error.message });
//   }
// });

// app.get("/get-html-report", async (req, res) => {
//   const requsername = req.query.username;
//   console.log(requsername);
//   //   res.sendFile(path.join(__dirname, "Reports", "cucumber-report.html"));
//   try {
//     const documentId = requsername + "_report_html"; // Use the correct document ID

//     // Fetch the HTML report from Couchbase
//     const result = await collection.get(documentId);

//     // Check if HTML content exists in the document
//     if (result && result.value && result.value.html) {
//       const htmlReport = result.value.html; // Assuming the HTML content is stored under the "html" field

//       res.send(htmlReport);
//     } else {
//       res.status(404).send("HTML report not found in Couchbase.");
//     }
//   } catch (err) {
//     console.error("Error fetching HTML report from Couchbase:", err);
//     res.status(500).send("Error fetching HTML report");
//   }
// });

// app.get("/get-json-report", async (req, res) => {
//   const requsername = req.query.username;
//   console.log(requsername);
//   try {
//     const documentId = requsername + "_report_json"; // Use the correct document ID

//     // Fetch the report from Couchbase
//     const result = await collection.get(documentId);
//     console.log(result);
//     // Check if report data exists in the document
//     //   if (result && result.value && result.value.json) {
//     //     const jsonReport = result.value.json; // Assuming the JSON report is stored under the "json" field

//     // Send the JSON report as the response
//     res.json(result);
//     //   } else {
//     //     res.status(404).send("JSON report not found in Couchbase.");
//     //   }
//   } catch (err) {
//     console.error("Error fetching JSON report from Couchbase:", err);
//     res.status(500).send("Error fetching JSON report");
//   }
// });

// app.get("/get-xml-report", async (req, res) => {
//   const requsername = req.query.username;
//   console.log(requsername);
//   try {
//     const documentId = requsername + "_report_xml"; // Use the correct document ID

//     // Fetch the XML report from Couchbase
//     const result = await collection.get(documentId);

//     // Check if XML content exists in the document
//     if (result && result.value && result.value.xml) {
//       const xmlReport = result.value.xml; // Assuming the XML content is stored under the "xml" field

//       // Send the XML content as the response
//       res.set("Content-Type", "application/xml");
//       res.send(xmlReport);
//     } else {
//       res.status(404).send("XML report not found in Couchbase.");
//     }
//   } catch (err) {
//     console.error("Error fetching XML report from Couchbase:", err);
//     res.status(500).send("Error fetching XML report");
//   }
// });

// app.listen(port, () => {
//   console.log(`Server listening at http://localhost:${port}`);
// });
