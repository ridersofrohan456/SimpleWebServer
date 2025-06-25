# MCP Server
I'm dookie at coding. Can I figure out how to write FastMCP in Rust?

## How does the Python SDK work?
Let's start at the [base server](https://github.com/modelcontextprotocol/python-sdk/blob/main/src/mcp/server/lowlevel/server.py). If we take a look at the run function, it:

### Step 1 - Server
await's [AsyncExitStack](https://github.com/sorcio/async_exit_stack/blob/master/async_exit_stack/_async_exit_stack.py), which seems to be some async context manager. Wtf is a context manager?

Uses "MethodType" from python, which represents a bound method. A quick example (pseudo-codey python-ish thing):
```aiignore
class SomeClass:
    field name;
    
    def SomeClass(name):
        this.name = name
    def hello(self):
        return "hello"
        
obj = MyClass("SomeName")

obj.say_hello = types.MethodType(hello, obj);

// obj.say_hello returns "SomeName"
```
Ah - context manager = ServerSession (stateful), which uses a read/write stream and some init options.

So the point of the AsyncConextManager is to essentially keep a stack of the __aexit__ methods (via wrapped MethodType objects), and this seems to be called during the __aexit__ method within the asyncExitStack - it'll pop all exit callbacks and then exit all nested server sessions.

**ServerSession**

Manages communication between the server and the client in the MCP framework. 

*BaseSession*

Implements an MCP "session" on top of read/write streams, and handles additional features: request/response linking, notifications, progress, etc. Is an async context manager. 

Read/write streams are "MemoryObjectReceiveStream" and "MemoryObjectSendStream"..


Oh wait a minute, I found a "ClientSession" that has some stuff I was looking for. It has defined methods like "call_tool", "list_Tools", "list_prompts", etc.

"call_tool" takes into account a name and some args, and those are passed into "CallToolRequestParams". That's then sent to a "write_stream" wrapped in a JSONRPCRequest (within a session) async. Once returned, the response model is validated before sending.

Within "_handle_post" and "_handle_get", there's a connect() method that's called to set up the streams. 


Okay so what's the bare minimum I need. I need a) a server connection to a tool, b) a connection to some AI?, c) a way to manage sessions and keep streams open, and d) a way to store some local "memory"? The python SDK uses "pydantic" for bindings.

Ahh this integrates with the Claude desktop application. The MCP server is installed within the Claude desktop app via "update_claude_config" method. There are "prompts" within FastMCP, which are actually maps to specific functions, i.e:
```aiignore
            @server.prompt()
            async def analyze_file(path: str) -> list[Message]:
                content = await read_file(path)
                return [
                    {
                        "role": "user",
                        "content": {
                            "type": "resource",
                            "resource": {
                                "uri": f"file://{path}",
                                "text": content
                            }
                        }
                    }
                ]
```

the MCPServer (base server) has access to these methods. FastMCP is moreso a warpper around MCPServer, to help reduce boilerplate and overall setup of a server.

Okay so immediate goal:
1. Create a _very_ basic server that can listen to Claude desktop requests and return a static response.