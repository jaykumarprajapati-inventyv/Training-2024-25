
async function connectAndInsert() {
  const clusterConnStr = "couchbases://cb.mij21cud055ga4ut.cloud.couchbase.com";
  const username = "jay";
  const password = "Jay$2833";

  try {
    console.log("⏳ Connecting to Couchbase Capella...");

    const cluster = await connect(clusterConnStr, {
      username: username,
      password: password,
      connectionOptions: {
        
        secure: true,
        trustCertificate: true, 
      },
      timeoutOptions: { connectTimeout: 60000 }, 
    });

    
    const bucketName = "testghirkins";
    const bucket = cluster.bucket(bucketName);
    const collection = bucket.scope("_default").collection("_default");
  } catch (error) {
    console.error("❌ Operation failed:", error.message);
    process.exit(1);
  }
}


connectAndInsert();















































































































































































