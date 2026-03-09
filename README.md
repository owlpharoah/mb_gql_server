


right now it will support basic lookup of these entities:
- Artist
- Release Group
- Releases
- Recordings

sample query:
```
query Test{
	artist(gid:"164f0d73-1234-4e2c-8743-d77bf2191051"){
		name
		releaseGroups(limit:2){
			name
			releases{
				name
				recordings{
					name
				}
			}
    }
	}
}
```
output:
```
{
	"data": {
		"artist": {
			"name": "Ye",
			"releaseGroups": [
				{
					"name": "2001 Demo Tape",
					"releases": [
						{
							"name": "2001 Demo Tape",
							"recordings": [
								{
									"name": "DJ Boom Freestyle (I Met Oprah)"
								},
								{
									"name": "DJ Boom Freestyle (Jigga That Nigga)"
								},
								{
									"name": "Dream Come True (All Falls Down Demo)"
								},
								{
									"name": "Family Business (2001 Unreleased)"
								},
								{
									"name": "Gotta Pose"
								},
								{
									"name": "Have It Your Way (Demo Version Of \"Bring Me Down)"
								},
								{
									"name": "Heartbeat (Instrumental)"
								},
								{
									"name": "Hey Mama (Original)"
								},
								{
									"name": "Home (Windy)"
								},
								{
									"name": "Jesus Walks (Original Version)"
								},
								{
									"name": "Know The Game (Unreleased)"
								},
								{
									"name": "Need To Know (Original \"Gangsta\" Version)"
								},
								{
									"name": "Never Letting Go (The Stalker Song)"
								},
								{
									"name": "Out Of Your Mind (Unmixed)"
								},
								{
									"name": "Wow (New Verse)"
								}
							]
						}
					]
				},
				{
					"name": "30 Hours",
					"releases": [
						{
							"name": "30 Hours",
							"recordings": [
								{
									"name": "30 Hours"
								}
							]
						}
					]
				}
			]
		}
	}
}
```
