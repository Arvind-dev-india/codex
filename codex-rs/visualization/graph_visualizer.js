// Global variables
let graphData = null;
let simulation = null;
let svg = null;
let showLabels = true;
let nodes = null;
let links = null;
let selectedNode = null;

// Color scheme for different symbol types
const colorScheme = {
    'Function': '#4CAF50',
    'Method': '#2196F3', 
    'Class': '#FF9800',
    'Struct': '#9C27B0',
    'Interface': '#00BCD4',
    'Enum': '#795548',
    'Variable': '#607D8B',
    'Constant': '#E91E63',
    'Module': '#FFC107',
    'Package': '#8BC34A'
};

// Initialize the visualization
function initVisualization() {
    const container = d3.select("#graph-container");
    const graphDiv = d3.select("#graph");
    
    // Create SVG element inside the graph div
    const containerRect = container.node().getBoundingClientRect();
    svg = graphDiv.append("svg")
        .attr("width", containerRect.width)
        .attr("height", containerRect.height);
    
    // Set up zoom behavior
    const zoom = d3.zoom()
        .scaleExtent([0.1, 3])
        .on("zoom", (event) => {
            svg.select("g").attr("transform", event.transform);
        });
    
    svg.call(zoom);
    
    // Add a group for all graph elements
    svg.append("g").attr("class", "graph-group");
    
    // Handle window resize
    window.addEventListener('resize', () => {
        const newRect = container.node().getBoundingClientRect();
        svg.attr("width", newRect.width)
           .attr("height", newRect.height);
        
        if (simulation) {
            simulation.force("center", d3.forceCenter(newRect.width / 2, newRect.height / 2));
            simulation.alpha(0.3).restart();
        }
    });
}

// Load and process graph data
function loadGraphData(data) {
    try {
        // Validate data structure
        if (!data || typeof data !== 'object') {
            throw new Error('Invalid data: expected an object');
        }
        
        if (!Array.isArray(data.nodes)) {
            throw new Error('Invalid data: missing or invalid nodes array');
        }
        
        if (!Array.isArray(data.edges)) {
            throw new Error('Invalid data: missing or invalid edges array');
        }
        
        if (data.nodes.length === 0) {
            throw new Error('No nodes found in the data');
        }
        
        if (data.edges.length === 0) {
            console.warn('No edges found in the data - only nodes will be displayed');
        }
        
        console.log(`Loading ${data.nodes.length} nodes and ${data.edges.length} edges`);
        
        // Create a set of valid node IDs for quick lookup
        const validNodeIds = new Set(data.nodes.map(node => node.id));
        
        // Filter out edges that reference non-existent nodes
        const validEdges = data.edges.filter(edge => {
            const sourceExists = validNodeIds.has(edge.source);
            const targetExists = validNodeIds.has(edge.target);
            
            if (!sourceExists || !targetExists) {
                console.warn(`Skipping invalid edge: ${edge.source} -> ${edge.target} (source exists: ${sourceExists}, target exists: ${targetExists})`);
                return false;
            }
            return true;
        });
        
        const filteredCount = data.edges.length - validEdges.length;
        console.log(`Filtered ${filteredCount} invalid edges, keeping ${validEdges.length} valid edges`);
        
        if (filteredCount > 0) {
            console.warn(`Warning: ${filteredCount} edges were filtered out because they reference non-existent nodes. This may indicate an issue with the data generation process.`);
        }
        
        graphData = {
            nodes: data.nodes,
            edges: validEdges
        };
        
        // Process nodes to add colors and sizes
        graphData.nodes.forEach(node => {
            node.color = colorScheme[node.symbol_type] || '#666666';
            node.size = getNodeSize(node.symbol_type);
        });
        
        // Populate the symbol selector dropdown
        populateSymbolSelector();
        
        // Populate the symbol type filter dropdown
        populateSymbolTypeFilter();
        
        // Enable the controls
        document.getElementById('symbolSelect').disabled = false;
        document.getElementById('symbolTypeFilter').disabled = false;
        
        // Create the force simulation
        createSimulation();
        
        // Render the graph
        renderGraph();
        
        // Update the symbol details to show success
        const symbolDetails = document.getElementById('symbol-details');
        const nodeTypes = [...new Set(graphData.nodes.map(node => node.symbol_type))].sort();
        
        let edgeInfo = `${graphData.edges.length}`;
        if (filteredCount > 0) {
            edgeInfo += ` (${filteredCount} filtered out)`;
        }
        
        symbolDetails.innerHTML = `
            <h2>Graph loaded successfully!</h2>
            <p><span class="label">Nodes:</span> ${graphData.nodes.length}</p>
            <p><span class="label">Edges:</span> ${edgeInfo}</p>
            <p><span class="label">Symbol Types:</span> ${nodeTypes.join(', ')}</p>
            ${filteredCount > 0 ? `<p><span class="label">Note:</span> ${filteredCount} edges were filtered out due to invalid node references.</p>` : ''}
            <p>Click on a node to view details or use the controls above to explore the graph.</p>
        `;
        
        console.log(`Successfully loaded ${graphData.nodes.length} nodes and ${graphData.edges.length} edges`);
        
    } catch (error) {
        console.error('Error loading graph data:', error);
        
        // Show error message to user
        const symbolDetails = document.getElementById('symbol-details');
        symbolDetails.innerHTML = `
            <h2>Error loading graph data</h2>
            <p><span class="label">Error:</span> ${error.message}</p>
            <p>Please check the console for more details and try loading a different JSON file.</p>
        `;
        
        // Reset controls
        document.getElementById('symbolSelect').disabled = true;
        document.getElementById('symbolTypeFilter').disabled = true;
        
        throw error; // Re-throw for caller to handle
    }
}

// Determine node size based on symbol type
function getNodeSize(symbolType) {
    const sizes = {
        'Class': 12,
        'Struct': 12,
        'Interface': 10,
        'Function': 8,
        'Method': 8,
        'Variable': 6,
        'Constant': 6,
        'Enum': 10,
        'Module': 14,
        'Package': 16
    };
    return sizes[symbolType] || 8;
}

// Create the force simulation
function createSimulation() {
    const container = d3.select("#graph-container");
    const containerRect = container.node().getBoundingClientRect();
    const width = containerRect.width;
    const height = containerRect.height;
    
    simulation = d3.forceSimulation(graphData.nodes)
        .force("link", d3.forceLink(graphData.edges).id(d => d.id).distance(100))
        .force("charge", d3.forceManyBody().strength(-300))
        .force("center", d3.forceCenter(width / 2, height / 2))
        .force("collision", d3.forceCollide().radius(d => d.size + 5));
}

// Render the graph
function renderGraph() {
    console.log('Rendering graph with', graphData.nodes.length, 'nodes and', graphData.edges.length, 'edges');
    
    const graphGroup = svg.select(".graph-group");
    
    // Clear existing elements
    graphGroup.selectAll("*").remove();
    
    // Create links
    const link = graphGroup.append("g")
        .attr("class", "links")
        .selectAll("line")
        .data(graphData.edges)
        .enter().append("line")
        .attr("class", "link")
        .style("stroke", "#999")
        .style("stroke-width", 1.5)
        .style("stroke-opacity", 0.6);
    
    console.log('Created', link.size(), 'links');
    
    // Create nodes
    const node = graphGroup.append("g")
        .attr("class", "nodes")
        .selectAll("g")
        .data(graphData.nodes)
        .enter().append("g")
        .attr("class", "node")
        .call(d3.drag()
            .on("start", dragstarted)
            .on("drag", dragged)
            .on("end", dragended));
    
    console.log('Created', node.size(), 'nodes');
    
    // Add circles to nodes
    node.append("circle")
        .attr("r", d => d.size)
        .attr("fill", d => d.color)
        .attr("stroke", "#fff")
        .attr("stroke-width", 1.5)
        .on("click", nodeClicked);
    
    // Add labels to nodes
    node.append("text")
        .attr("dx", d => d.size + 5)
        .attr("dy", ".35em")
        .style("font-size", "10px")
        .style("pointer-events", "none")
        .text(d => d.name)
        .style("display", showLabels ? "block" : "none");
    
    // Store references for later use
    nodes = node;
    links = link;
    
    // Update positions on simulation tick
    simulation.on("tick", () => {
        link
            .attr("x1", d => d.source.x)
            .attr("y1", d => d.source.y)
            .attr("x2", d => d.target.x)
            .attr("y2", d => d.target.y);
        
        node
            .attr("transform", d => `translate(${d.x},${d.y})`);
    });
    
    console.log('Graph rendering complete');
}

// Drag functions
function dragstarted(event, d) {
    if (!event.active) simulation.alphaTarget(0.3).restart();
    d.fx = d.x;
    d.fy = d.y;
}

function dragged(event, d) {
    d.fx = event.x;
    d.fy = event.y;
}

function dragended(event, d) {
    if (!event.active) simulation.alphaTarget(0);
    d.fx = null;
    d.fy = null;
}

// Node click handler
function nodeClicked(event, d) {
    selectNode(d);
    
    // Update the dropdown to show the selected symbol
    const symbolSelect = document.getElementById('symbolSelect');
    symbolSelect.value = d.id;
}

// Show node information in the info panel
function showNodeInfo(node) {
    const symbolDetails = document.getElementById('symbol-details');
    
    const connectedEdges = graphData.edges.filter(e => 
        e.source.id === node.id || e.target.id === node.id
    );
    
    symbolDetails.innerHTML = `
        <h2>${node.name}</h2>
        <p><span class="label">Type:</span> ${node.symbol_type}</p>
        <p><span class="label">File:</span> ${node.file_path}</p>
        <p><span class="label">Lines:</span> ${node.start_line} - ${node.end_line}</p>
        <p><span class="label">FQN:</span> ${node.id}</p>
        <p><span class="label">Connections:</span> ${connectedEdges.length}</p>
        ${node.parent ? `<p><span class="label">Parent:</span> ${node.parent}</p>` : ''}
    `;
}

// Highlight connections for a selected node
function highlightConnections(selectedNode) {
    // Reset all styles
    svg.selectAll(".link").style("stroke", "#999").style("stroke-width", 2);
    svg.selectAll(".node circle").style("opacity", 0.3);
    
    // Highlight the selected node
    svg.selectAll(".node").filter(d => d.id === selectedNode.id)
        .select("circle").style("opacity", 1);
    
    // Find and highlight connected nodes and edges
    graphData.edges.forEach(edge => {
        if (edge.source.id === selectedNode.id || edge.target.id === selectedNode.id) {
            // Highlight the edge
            svg.selectAll(".link").filter(d => d === edge)
                .style("stroke", "#ff6b6b").style("stroke-width", 3);
            
            // Highlight connected nodes
            const connectedNodeId = edge.source.id === selectedNode.id ? edge.target.id : edge.source.id;
            svg.selectAll(".node").filter(d => d.id === connectedNodeId)
                .select("circle").style("opacity", 1);
        }
    });
}

// Reset view
function resetView() {
    if (!svg) return;
    
    // Clear selections
    svg.selectAll(".selected").classed("selected", false);
    svg.selectAll(".link").style("stroke", "#999").style("stroke-width", 1.5).style("opacity", 0.6);
    svg.selectAll(".node").style("opacity", 1).style("display", "block");
    
    // Reset dropdowns and search
    document.getElementById('symbolSelect').value = '';
    document.getElementById('symbolTypeFilter').value = '';
    document.getElementById('search').value = '';
    
    // Reset symbol details
    document.getElementById('symbol-details').innerHTML = '<h2>Select a node to view details</h2>';
    
    // Reset zoom
    svg.transition().duration(750).call(
        d3.zoom().transform,
        d3.zoomIdentity
    );
    
    selectedNode = null;
}

// Toggle labels
function toggleLabels() {
    showLabels = !showLabels;
    svg.selectAll(".node text")
        .style("display", showLabels ? "block" : "none");
}

// File input handler
document.getElementById('jsonFile').addEventListener('change', function(event) {
    const file = event.target.files[0];
    if (file) {
        const reader = new FileReader();
        reader.onload = function(e) {
            try {
                const data = JSON.parse(e.target.result);
                loadGraphData(data);
            } catch (error) {
                console.error('Error loading file:', error);
                const symbolDetails = document.getElementById('symbol-details');
                symbolDetails.innerHTML = `
                    <h2>Error loading file</h2>
                    <p><span class="label">File:</span> ${file.name}</p>
                    <p><span class="label">Error:</span> ${error.message}</p>
                    <p>Please check that the file contains valid JSON with nodes and edges arrays.</p>
                `;
            }
        };
        reader.readAsText(file);
    }
});

// Populate symbol selector dropdown
function populateSymbolSelector() {
    const symbolSelect = document.getElementById('symbolSelect');
    
    // Clear existing options except the first one
    symbolSelect.innerHTML = '<option value="">-- Select a symbol --</option>';
    
    // Sort symbols by name for easier browsing
    const sortedNodes = [...graphData.nodes].sort((a, b) => a.name.localeCompare(b.name));
    
    sortedNodes.forEach(node => {
        const option = document.createElement('option');
        option.value = node.id;
        option.textContent = `${node.name} (${node.symbol_type}) - ${node.file_path}`;
        symbolSelect.appendChild(option);
    });
}

// Populate symbol type filter dropdown
function populateSymbolTypeFilter() {
    const typeFilter = document.getElementById('symbolTypeFilter');
    
    // Clear existing options except the first one
    typeFilter.innerHTML = '<option value="">All Types</option>';
    
    // Get unique symbol types
    const symbolTypes = [...new Set(graphData.nodes.map(node => node.symbol_type))].sort();
    
    symbolTypes.forEach(type => {
        const option = document.createElement('option');
        option.value = type;
        option.textContent = type;
        typeFilter.appendChild(option);
    });
}

// Focus on a selected symbol
function focusOnSymbol() {
    const symbolSelect = document.getElementById('symbolSelect');
    const selectedId = symbolSelect.value;
    
    if (!selectedId) {
        // If no symbol selected, reset view
        resetView();
        return;
    }
    
    const node = graphData.nodes.find(n => n.id === selectedId);
    if (!node) {
        console.error('Node not found:', selectedId);
        return;
    }
    
    console.log('Focusing on symbol:', node.name, node.symbol_type);
    
    // Simulate a click on the node
    selectNode(node);
    
    // Center the view on the node
    centerOnNode(node);
}

// Select a node and show its information
function selectNode(node) {
    selectedNode = node;
    
    // Remove previous selection
    if (svg) {
        svg.selectAll(".node").classed("selected", false);
        svg.selectAll(".node").filter(d => d.id === node.id).classed("selected", true);
    }
    
    // Show node information
    showNodeInfo(node);
    
    // Highlight connections
    highlightConnections(node);
}

// Center the view on a specific node
function centerOnNode(node) {
    if (!svg || !node.x || !node.y) return;
    
    const width = svg.node().getBoundingClientRect().width;
    const height = svg.node().getBoundingClientRect().height;
    
    const scale = 1.5;
    const x = -node.x * scale + width / 2;
    const y = -node.y * scale + height / 2;
    
    svg.transition()
        .duration(750)
        .call(
            d3.zoom().transform,
            d3.zoomIdentity.translate(x, y).scale(scale)
        );
}

// Filter nodes by type
function filterByType() {
    const typeFilter = document.getElementById('symbolTypeFilter');
    const selectedType = typeFilter.value;
    
    if (!svg) return;
    
    if (selectedType === '') {
        // Show all nodes
        svg.selectAll(".node").style("display", "block");
        svg.selectAll(".link").style("display", "block");
    } else {
        // Hide nodes that don't match the selected type
        svg.selectAll(".node").style("display", d => d.symbol_type === selectedType ? "block" : "none");
        
        // Hide links where either source or target is hidden
        svg.selectAll(".link").style("display", d => {
            const sourceVisible = d.source.symbol_type === selectedType;
            const targetVisible = d.target.symbol_type === selectedType;
            return sourceVisible && targetVisible ? "block" : "none";
        });
    }
}

// Search for symbols
function searchSymbols() {
    const searchInput = document.getElementById('search');
    const searchTerm = searchInput.value.toLowerCase();
    
    if (!svg) return;
    
    if (searchTerm === '') {
        // Show all nodes
        svg.selectAll(".node").style("opacity", 1);
        svg.selectAll(".link").style("opacity", 0.6);
    } else {
        // Highlight matching nodes
        svg.selectAll(".node").style("opacity", d => {
            const nameMatch = d.name.toLowerCase().includes(searchTerm);
            const typeMatch = d.symbol_type.toLowerCase().includes(searchTerm);
            const fileMatch = d.file_path.toLowerCase().includes(searchTerm);
            return nameMatch || typeMatch || fileMatch ? 1 : 0.2;
        });
        
        // Dim links to non-matching nodes
        svg.selectAll(".link").style("opacity", d => {
            const sourceMatch = d.source.name.toLowerCase().includes(searchTerm) || 
                               d.source.symbol_type.toLowerCase().includes(searchTerm);
            const targetMatch = d.target.name.toLowerCase().includes(searchTerm) || 
                               d.target.symbol_type.toLowerCase().includes(searchTerm);
            return sourceMatch || targetMatch ? 0.6 : 0.1;
        });
    }
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', function() {
    initVisualization();
    
    // Set up event listeners
    document.getElementById('symbolSelect').addEventListener('change', focusOnSymbol);
    document.getElementById('symbolTypeFilter').addEventListener('change', filterByType);
    document.getElementById('search').addEventListener('input', searchSymbols);
    document.getElementById('reset-zoom').addEventListener('click', resetView);
    document.getElementById('toggle-labels').addEventListener('click', toggleLabels);
});