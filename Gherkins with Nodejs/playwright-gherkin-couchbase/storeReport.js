import { connect } from "couchbase";
import dotenv from "dotenv";
import fs from "fs";

dotenv.config();

const clusterConnStr = "couchbases://cb.mij21cud055ga4ut.cloud.couchbase.com";
const username = "jay";
const password = "Jay$2833";

// Read the results.json file
const reportData = JSON.parse(fs.readFileSync("./results.json", "utf-8"));

// Connect to Couchbase with SSL (couchbases://) and ensure SSL options are set
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

// Function to store the report in Couchbase
async function storeReportWithRetry(retries = 3) {
  let attempt = 0;
  while (attempt < retries) {
    try {
      console.log("Attempting to store report...");
      await collection.upsert("latestReport", reportData);
      console.log("Report successfully stored in Couchbase.");
      return; // Exit after success
    } catch (error) {
      console.error(`Error storing report (attempt ${attempt + 1}):`, error);
      if (attempt < retries - 1) {
        console.log("Retrying...");
      }
      attempt++;
    }
  }
  console.error("Failed to store report after multiple attempts.");
}

storeReportWithRetry();
