# protocol version management

CITA use protocol version number to activate hard fork for upgrading. This
contract implement setter and getter for protocol version number.

## Interface

### Operations

<table style = "text-align: center;">
  <tr>
    <th>Function Name</th>
    <th>Permission Required</th>
    <th>Input Parameters</th>
    <th>Returned Values</th>
    <th>Discription</th>
  </tr>
  <tr>
    <td>
      setVersion(version) <br/>
      <strong>Set an version</strong>
    </td>
    <td>Admin</td>
    <td>
      new version number (uint32)
    <td>None</td>
    <td>Set an new version number</td>
  </tr>
</table>

#### Query

Query don't require permission at all.

<table style = "text-align: center;">
  <tr>
    <th>Function Name</th>
    <th>Input Parameters</th>
    <th>Returned Values</th>
    <th>Discription</th>
  </tr>
  <tr>
    <td>
      getVersion() <br/>
      <strong>Get current version</strong>
    </td>
    <td>
        None
    </td>
    <td>Current version (uint32)</td>
    <td>Get current version</td>
  </tr>
</table>
