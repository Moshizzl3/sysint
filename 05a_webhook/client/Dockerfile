# Use the official Node.js v21.7.1 image as base
FROM --platform=linux/amd64 node:21.7.1

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy package.json and package-lock.json (if available) to the working directory
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application code to the working directory
COPY . .

# Expose port 80
EXPOSE 80

# Command to run your application
CMD ["node", "app.js"]  
