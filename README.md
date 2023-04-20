<a name="readme-top"></a>
# Vivace

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li><a href="#build-from-source">Build From Source</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#examples">Examples</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Command line tool in Rust to download songs from Youtube to whatever encoding you want

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- Build from source -->
## Build from source 

To build the project from source, run `cargo build --release` within the project directory

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE -->
## Usage

```
vivace [OPTIONS] --url <URL> --output-file <OUTPUT_FILE>

Options:
  -u, --url <URL>                  Video URL
  -o, --output-file <OUTPUT_FILE>  Output file name
  -c, --chunk-size <CHUNK_SIZE>    Chunk size for partial requests
  -h, --help                       Print help
  -V, --version                    Print version
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- EXAMPLES -->
## Examples

Download a song to given output file with .mp3 extension

`vivace --url https://www.youtube.com/watch?v=dQw4w9WgXcQ --output-file my-favourite-song.mp3`

Download a song to given output file with .wav extension

`vivace --url https://www.youtube.com/watch?v=dQw4w9WgXcQ --output-file my-favourite-song.wav`

Download a song to given output file and specify chunk size (in bytes) for the partial requests to the server

`vivace --url https://www.youtube.com/watch?v=dQw4w9WgXcQ --output-file my-favourite-song.mp3 -c 10240`

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Pavlos Smith - paulsmith4561+at+gmail.com

<p align="right">(<a href="#readme-top">back to top</a>)</p>

