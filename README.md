# ip-rep

Rust cli tooling for checking IP reputation from several sources

## Sources

The CLI allows grabbing data from multiple sources.

### Geolocation Data Sources

#### Services integrated into the CLI

1. **[MaxMind GeoIP2 Database:](https://dev.maxmind.com/geoip/geolite2-free-geolocation-data)** Provides accurate geolocation data based on IP addresses.
   - Signup: [Sign up to GeoLite2 here](https://www.maxmind.com/en/geolite2/signup). License keys are managed [here](https://www.maxmind.com/en/accounts/1001485/license-key/).
   - Requests are made directly to their GeoLite2 City endpoint, documented [here](https://www.maxmind.com/en/accounts/1001485/license-key/) once you've logged in.
2. **[IPinfo.io API:](https://ipinfo.io/)** Offers geolocation data including country, region, city, postal code, latitude, longitude, etc.
   - Free up to 50k requests per month
   - Includes a rust sdk, integrated into the CLI (`ipinfo = "3.0.0"`)

#### Services not yet integrated into the CLI

2. **[IP2Location Database:](https://www.ip2location.com/)** Offers geolocation data including country, region, city, latitude, longitude, ZIP code, timezone, and more.
   - No free tier is offered, and the pricing is expensive, starting at [$980 per year](https://www.ip2location.com/buy-online)
   - The secondary service is the IP2Proxy Database. This is significantly more expensive, starting at $7980
3. **IPinfo.io API:** Offers geolocation data including country, region, city, postal code, latitude, longitude, and more via an API.
4. **ipapi.com API:** Provides geolocation data along with ASN, ISP, country, city, and more.
5. **FreeGeoIP API:** A free API to lookup geolocation data of an IP address.

### IP Reputation Data Sources:

1. **AbuseIPDB:** Offers IP reputation data including reports of abusive activities associated with an IP address.
2. **AlienVault OTX:** Provides threat intelligence including IP reputation data based on crowdsourced contributions.
3. **IBM X-Force Exchange:** Offers IP reputation data and threat intelligence based on IBM's security research.
4. **VirusTotal:** Provides IP reputation data based on malware reports and other security indicators.
5. **Talos Intelligence:** Offers IP reputation data and threat intelligence based on Cisco's security research.
6. **IPVoid:** Provides various IP reputation checks including blacklist, WHOIS, DNS, and geolocation.
