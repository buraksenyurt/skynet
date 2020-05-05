using System;
using Microsoft.EntityFrameworkCore.Migrations;

namespace MusicShop.Migrations
{
    public partial class SchemaChange : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.AddColumn<string>(
                name: "Biography",
                table: "Artist",
                nullable: true);

            migrationBuilder.AddColumn<byte[]>(
                name: "Cover",
                table: "Album",
                nullable: true);
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropColumn(
                name: "Biography",
                table: "Artist");

            migrationBuilder.DropColumn(
                name: "Cover",
                table: "Album");
        }
    }
}
