"use client";

import { useState } from 'react';

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Button } from "@/components/ui/button"
import { Upload } from "lucide-react"

export default function Home() {
  const [contractAddress, setContractAddress] = useState('');
  const [file, setFile] = useState(null);
  const [isDragging, setIsDragging] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleDragOver = (e) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = () => {
    setIsDragging(false);
  };

  const handleDrop = (e) => {
    e.preventDefault();
    setIsDragging(false);
    
    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      setFile(e.dataTransfer.files[0]);
    }
  };

  const handleFileChange = (e) => {
    if (e.target.files && e.target.files[0]) {
      setFile(e.target.files[0]);
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsSubmitting(true);

    try {
      // Here you would typically send the data to your backend
      // Example using FormData:
      const formData = new FormData();
      formData.append('contractAddress', contractAddress);
      if (file) {
        formData.append('file', file);
      }

      // Simulated API call
      await new Promise(resolve => setTimeout(resolve, 1000));
      console.log('Form submitted:', {
        contractAddress,
        fileName: file?.name
      });

      // Reset form after successful submission
      setContractAddress('');
      setFile(null);
    } catch (error) {
      console.error('Error submitting form:', error);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex justify-center items-center min-h-screen p-4 bg-gray-950">
      <Card className="w-full max-w-md border-violet-500 bg-gray-900 text-white">
        <CardHeader>
          <CardTitle className="text-violet-400">Contract Upload</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="contractAddress" className="text-violet-300">Contract Address</Label>
              <Input
                id="contractAddress"
                placeholder="Enter contract address"
                value={contractAddress}
                onChange={(e) => setContractAddress(e.target.value)}
                className="bg-gray-800 border-violet-500 text-white placeholder:text-gray-400 focus:ring-violet-500 focus:border-violet-500"
                required
              />
            </div>
            
            <div className="space-y-2">
              <Label className="text-violet-300">Upload File</Label>
              <div
                onDragOver={handleDragOver}
                onDragLeave={handleDragLeave}
                onDrop={handleDrop}
                className={`border-2 border-dashed rounded-lg p-6 text-center cursor-pointer transition-all ${
                  isDragging 
                    ? 'border-violet-400 bg-violet-500/10' 
                    : 'border-violet-500 hover:border-violet-400 hover:bg-violet-500/5'
                }`}
              >
                <input
                  type="file"
                  onChange={handleFileChange}
                  className="hidden"
                  id="fileInput"
                  required
                />
                <label htmlFor="fileInput" className="cursor-pointer">
                  <div className="flex flex-col items-center gap-2">
                    <Upload className="w-8 h-8 text-violet-400" />
                    <div className="text-sm text-gray-300">
                      {file ? file.name : 'Drag and drop a file here, or click to select'}
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <Button
              type="submit"
              disabled={isSubmitting || !contractAddress || !file}
              className="w-full rounded-full bg-violet-600 hover:bg-violet-700 text-white font-medium py-2 px-4 transition-colors"
            >
              {isSubmitting ? (
                <div className="flex items-center justify-center gap-2">
                  <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
                  Submitting...
                </div>
              ) : (
                'Submit'
              )}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
