#!/usr/bin/env python3
"""Convert ESP application markdown to PDF"""
from markdown_pdf import MarkdownPdf, Section
import os

md_path = "C:/Users/molel/grant/funding/esp_application_pretxsim.md"
pdf_path = "C:/Users/molel/grant/funding/esp_application_pretxsim.pdf"

with open(md_path, "r", encoding="utf-8") as f:
    md_text = f.read()

pdf = MarkdownPdf()
pdf.add_section(Section(md_text))
pdf.save(pdf_path)

print(f"PDF created: {pdf_path} ({os.path.getsize(pdf_path)} bytes)")
